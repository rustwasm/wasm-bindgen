use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTableElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTableElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTableElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTableElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
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
    impl core::ops::Deref for HtmlTableElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTableElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTableElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTableElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTableElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTableElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTableElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTableElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTableElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTableElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTableElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTableElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTableElement {
            HtmlTableElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTableElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTableElement> for HtmlTableElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTableElement {
            self
        }
    }
    impl From<HtmlTableElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTableElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTableElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTableElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTableElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTableElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTableElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTableElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTableElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTableElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTableElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableElement> for Element {
    #[inline]
    fn from(obj: HtmlTableElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTableElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableElement> for Node {
    #[inline]
    fn from(obj: HtmlTableElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTableElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTableElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTableElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTableElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTableElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_caption_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `createCaption()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn create_caption(&self) -> HtmlElement {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_caption_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_caption_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_caption_HTMLTableElement(self_)
            };
            <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_t_body_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `createTBody()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn create_t_body(&self) -> HtmlElement {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_t_body_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_t_body_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_t_body_HTMLTableElement(self_)
            };
            <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_t_foot_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `createTFoot()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn create_t_foot(&self) -> HtmlElement {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_t_foot_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_t_foot_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_t_foot_HTMLTableElement(self_)
            };
            <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_t_head_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `createTHead()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn create_t_head(&self) -> HtmlElement {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_t_head_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_t_head_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_t_head_HTMLTableElement(self_)
            };
            <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_caption_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteCaption()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn delete_caption(&self) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_caption_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_caption_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_caption_HTMLTableElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_row_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn delete_row(&self, index: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_row_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_row_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_delete_row_HTMLTableElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_t_foot_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteTFoot()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn delete_t_foot(&self) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_t_foot_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_t_foot_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_t_foot_HTMLTableElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_t_head_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteTHead()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn delete_t_head(&self) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_t_head_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_t_head_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_t_head_HTMLTableElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_row_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn insert_row(&self) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_row_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_row_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_insert_row_HTMLTableElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_row_with_index_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn insert_row_with_index(
        &self,
        index: i32,
    ) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_row_with_index_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_row_with_index_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_row_with_index_HTMLTableElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_caption_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<HtmlTableCaptionElement> as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `caption` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn caption(&self) -> Option<HtmlTableCaptionElement> {
        #[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_caption_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlTableCaptionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_caption_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlTableCaptionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_caption_HTMLTableElement(self_)
            };
            <Option<HtmlTableCaptionElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_caption_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<&HtmlTableCaptionElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `caption` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_caption(&self, caption: Option<&HtmlTableCaptionElement>) {
        #[cfg(all(feature = "HtmlTableCaptionElement", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_caption_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                caption : < Option < & HtmlTableCaptionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_caption_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            caption: <Option<&HtmlTableCaptionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(caption);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let caption = < Option < & HtmlTableCaptionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( caption ) ;
                __widl_f_set_caption_HTMLTableElement(self_, caption)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_t_head_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<HtmlTableSectionElement> as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `tHead` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn t_head(&self) -> Option<HtmlTableSectionElement> {
        #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_t_head_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_t_head_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_t_head_HTMLTableElement(self_)
            };
            <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_t_head_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<&HtmlTableSectionElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `tHead` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_t_head(&self, t_head: Option<&HtmlTableSectionElement>) {
        #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_t_head_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                t_head : < Option < & HtmlTableSectionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_t_head_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            t_head: <Option<&HtmlTableSectionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(t_head);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let t_head = < Option < & HtmlTableSectionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( t_head ) ;
                __widl_f_set_t_head_HTMLTableElement(self_, t_head)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_t_foot_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<HtmlTableSectionElement> as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `tFoot` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn t_foot(&self) -> Option<HtmlTableSectionElement> {
        #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_t_foot_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_t_foot_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_t_foot_HTMLTableElement(self_)
            };
            <Option<HtmlTableSectionElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_t_foot_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <Option<&HtmlTableSectionElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `tFoot` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_t_foot(&self, t_foot: Option<&HtmlTableSectionElement>) {
        #[cfg(all(feature = "HtmlTableElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_t_foot_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                t_foot : < Option < & HtmlTableSectionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_t_foot_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            t_foot: <Option<&HtmlTableSectionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(t_foot);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let t_foot = < Option < & HtmlTableSectionElement > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( t_foot ) ;
                __widl_f_set_t_foot_HTMLTableElement(self_, t_foot)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_t_bodies_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `tBodies` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn t_bodies(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_t_bodies_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_t_bodies_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_t_bodies_HTMLTableElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rows_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn rows(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rows_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rows_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rows_HTMLTableElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLTableElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_border_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn border(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_border_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_border_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_border_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_border_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_border(&self, border: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_border_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_border_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let border = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                __widl_f_set_border_HTMLTableElement(self_, border)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frame_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `frame` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn frame(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frame_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frame_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frame_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_frame_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `frame` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_frame(&self, frame: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_frame_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frame: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_frame_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frame: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(frame);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frame = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frame);
                __widl_f_set_frame_HTMLTableElement(self_, frame)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rules_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `rules` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn rules(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rules_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rules_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rules_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rules_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `rules` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_rules(&self, rules: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rules_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rules: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rules_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rules: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rules);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rules = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rules);
                __widl_f_set_rules_HTMLTableElement(self_, rules)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_summary_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `summary` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn summary(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_summary_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_summary_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_summary_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_summary_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `summary` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_summary(&self, summary: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_summary_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                summary: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_summary_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            summary: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(summary);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let summary = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(summary);
                __widl_f_set_summary_HTMLTableElement(self_, summary)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLTableElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bg_color_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn bg_color(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bg_color_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bg_color_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bg_color_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_bg_color_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_bg_color(&self, bg_color: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_bg_color_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_bg_color_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(bg_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_set_bg_color_HTMLTableElement(self_, bg_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cell_padding_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `cellPadding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn cell_padding(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cell_padding_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cell_padding_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cell_padding_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cell_padding_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `cellPadding` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_cell_padding(&self, cell_padding: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cell_padding_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cell_padding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cell_padding_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cell_padding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cell_padding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cell_padding =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cell_padding);
                __widl_f_set_cell_padding_HTMLTableElement(self_, cell_padding)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cell_spacing_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `cellSpacing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn cell_spacing(&self) -> String {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cell_spacing_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cell_spacing_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cell_spacing_HTMLTableElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cell_spacing_HTMLTableElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableElement {
    #[cfg(all(feature = "HtmlTableElement",))]
    #[allow(bad_style)]
    #[doc = "The `cellSpacing` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)\n\n*This API requires the following crate features to be activated: `HtmlTableElement`*"]
    #[allow(clippy::all)]
    pub fn set_cell_spacing(&self, cell_spacing: &str) {
        #[cfg(all(feature = "HtmlTableElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cell_spacing_HTMLTableElement(
                self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cell_spacing: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cell_spacing_HTMLTableElement(
            self_: <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cell_spacing: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cell_spacing);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cell_spacing =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cell_spacing);
                __widl_f_set_cell_spacing_HTMLTableElement(self_, cell_spacing)
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
pub static __WASM_BINDGEN_GENERATED_4cf1c21d588fda76: [u8; 3371usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE9\x0C\0\0\0\0%\0\0\x02\x10HTMLTableElement\"__widl_instanceof_HTMLTableElement\0\0\0\0(__widl_f_create_caption_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\rcreateCaption\0\0\0'__widl_f_create_t_body_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\x0BcreateTBody\0\0\0'__widl_f_create_t_foot_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\x0BcreateTFoot\0\0\0'__widl_f_create_t_head_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\x0BcreateTHead\0\0\0(__widl_f_delete_caption_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\rdeleteCaption\0\0\0$__widl_f_delete_row_HTMLTableElement\x01\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x02\x05self_\x05index\tdeleteRow\0\0\0'__widl_f_delete_t_foot_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\x0BdeleteTFoot\0\0\0'__widl_f_delete_t_head_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\x0BdeleteTHead\0\0\0$__widl_f_insert_row_HTMLTableElement\x01\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x01\x05self_\tinsertRow\0\0\0/__widl_f_insert_row_with_index_HTMLTableElement\x01\0\0\x01\x10HTMLTableElement\x01\0\0\x01\x02\x05self_\x05index\tinsertRow\0\0\0!__widl_f_caption_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x07caption\x01\x01\x05self_\x07caption\0\0\0%__widl_f_set_caption_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x07caption\x01\x02\x05self_\x07caption\x07caption\0\0\0 __widl_f_t_head_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05tHead\x01\x01\x05self_\x05tHead\0\0\0$__widl_f_set_t_head_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05tHead\x01\x02\x05self_\x06t_head\x05tHead\0\0\0 __widl_f_t_foot_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05tFoot\x01\x01\x05self_\x05tFoot\0\0\0$__widl_f_set_t_foot_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05tFoot\x01\x02\x05self_\x06t_foot\x05tFoot\0\0\0\"__widl_f_t_bodies_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x07tBodies\x01\x01\x05self_\x07tBodies\0\0\0\x1E__widl_f_rows_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x04rows\x01\x01\x05self_\x04rows\0\0\0\x1F__widl_f_align_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0#__widl_f_set_align_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0 __widl_f_border_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x06border\x01\x01\x05self_\x06border\0\0\0$__widl_f_set_border_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x06border\x01\x02\x05self_\x06border\x06border\0\0\0\x1F__widl_f_frame_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05frame\x01\x01\x05self_\x05frame\0\0\0#__widl_f_set_frame_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05frame\x01\x02\x05self_\x05frame\x05frame\0\0\0\x1F__widl_f_rules_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05rules\x01\x01\x05self_\x05rules\0\0\0#__widl_f_set_rules_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05rules\x01\x02\x05self_\x05rules\x05rules\0\0\0!__widl_f_summary_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x07summary\x01\x01\x05self_\x07summary\0\0\0%__widl_f_set_summary_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x07summary\x01\x02\x05self_\x07summary\x07summary\0\0\0\x1F__widl_f_width_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0#__widl_f_set_width_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\"__widl_f_bg_color_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x07bgColor\x01\x01\x05self_\x07bgColor\0\0\0&__widl_f_set_bg_color_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x07bgColor\x01\x02\x05self_\x08bg_color\x07bgColor\0\0\0&__widl_f_cell_padding_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x0BcellPadding\x01\x01\x05self_\x0BcellPadding\0\0\0*__widl_f_set_cell_padding_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x0BcellPadding\x01\x02\x05self_\x0Ccell_padding\x0BcellPadding\0\0\0&__widl_f_cell_spacing_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x01\x0BcellSpacing\x01\x01\x05self_\x0BcellSpacing\0\0\0*__widl_f_set_cell_spacing_HTMLTableElement\0\0\0\x01\x10HTMLTableElement\x01\0\x02\x0BcellSpacing\x01\x02\x05self_\x0Ccell_spacing\x0BcellSpacing\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
