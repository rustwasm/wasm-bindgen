use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTableSectionElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTableSectionElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTableSectionElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTableSectionElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
            inform(83u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlTableSectionElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTableSectionElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTableSectionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTableSectionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTableSectionElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTableSectionElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTableSectionElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTableSectionElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTableSectionElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTableSectionElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTableSectionElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTableSectionElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTableSectionElement {
            HtmlTableSectionElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTableSectionElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTableSectionElement> for HtmlTableSectionElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTableSectionElement {
            self
        }
    }
    impl From<HtmlTableSectionElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTableSectionElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTableSectionElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTableSectionElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTableSectionElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTableSectionElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTableSectionElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTableSectionElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTableSectionElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTableSectionElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTableSectionElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableSectionElement> for Element {
    #[inline]
    fn from(obj: HtmlTableSectionElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTableSectionElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableSectionElement> for Node {
    #[inline]
    fn from(obj: HtmlTableSectionElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTableSectionElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableSectionElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTableSectionElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTableSectionElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableSectionElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTableSectionElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTableSectionElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_row_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn delete_row(&self, index: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_row_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_row_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_delete_row_HTMLTableSectionElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_row_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn insert_row(&self) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_row_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_row_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_insert_row_HTMLTableSectionElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_row_with_index_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertRow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn insert_row_with_index(
        &self,
        index: i32,
    ) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_row_with_index_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_row_with_index_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_row_with_index_HTMLTableSectionElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rows_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn rows(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rows_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rows_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_rows_HTMLTableSectionElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_align_HTMLTableSectionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLTableSectionElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn ch(&self) -> String {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_ch_HTMLTableSectionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch(&self, ch: &str) {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let ch = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch);
                __widl_f_set_ch_HTMLTableSectionElement(self_, ch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_off_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn ch_off(&self) -> String {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_off_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_off_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_ch_off_HTMLTableSectionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_off_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch_off(&self, ch_off: &str) {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_off_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch_off: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_off_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let ch_off = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch_off);
                __widl_f_set_ch_off_HTMLTableSectionElement(self_, ch_off)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_v_align_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn v_align(&self) -> String {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_v_align_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_v_align_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_v_align_HTMLTableSectionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableSectionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_v_align_HTMLTableSectionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableSectionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableSectionElement {
    #[cfg(all(feature = "HtmlTableSectionElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    #[allow(clippy::all)]
    pub fn set_v_align(&self, v_align: &str) {
        #[cfg(all(feature = "HtmlTableSectionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_v_align_HTMLTableSectionElement(
                self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                v_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_v_align_HTMLTableSectionElement(
            self_: <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableSectionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let v_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(v_align);
                __widl_f_set_v_align_HTMLTableSectionElement(self_, v_align)
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
pub static __WASM_BINDGEN_GENERATED_9d67c417f3d90cac: [u8; 1352usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x06\x05\0\0\0\0\r\0\0\x02\x17HTMLTableSectionElement)__widl_instanceof_HTMLTableSectionElement\0\0\0\0+__widl_f_delete_row_HTMLTableSectionElement\x01\0\0\x01\x17HTMLTableSectionElement\x01\0\0\x01\x02\x05self_\x05index\tdeleteRow\0\0\0+__widl_f_insert_row_HTMLTableSectionElement\x01\0\0\x01\x17HTMLTableSectionElement\x01\0\0\x01\x01\x05self_\tinsertRow\0\0\06__widl_f_insert_row_with_index_HTMLTableSectionElement\x01\0\0\x01\x17HTMLTableSectionElement\x01\0\0\x01\x02\x05self_\x05index\tinsertRow\0\0\0%__widl_f_rows_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x01\x04rows\x01\x01\x05self_\x04rows\0\0\0&__widl_f_align_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0*__widl_f_set_align_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0#__widl_f_ch_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x01\x02ch\x01\x01\x05self_\x02ch\0\0\0'__widl_f_set_ch_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x02\x02ch\x01\x02\x05self_\x02ch\x02ch\0\0\0'__widl_f_ch_off_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x01\x05chOff\x01\x01\x05self_\x05chOff\0\0\0+__widl_f_set_ch_off_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x02\x05chOff\x01\x02\x05self_\x06ch_off\x05chOff\0\0\0(__widl_f_v_align_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x01\x06vAlign\x01\x01\x05self_\x06vAlign\0\0\0,__widl_f_set_v_align_HTMLTableSectionElement\0\0\0\x01\x17HTMLTableSectionElement\x01\0\x02\x06vAlign\x01\x02\x05self_\x07v_align\x06vAlign\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
