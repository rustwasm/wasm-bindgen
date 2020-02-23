use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTableRowElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTableRowElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTableRowElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTableRowElement {
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
            inform(82u32);
            inform(111u32);
            inform(119u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlTableRowElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTableRowElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTableRowElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTableRowElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTableRowElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTableRowElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTableRowElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTableRowElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTableRowElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTableRowElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTableRowElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTableRowElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTableRowElement {
            HtmlTableRowElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTableRowElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTableRowElement> for HtmlTableRowElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTableRowElement {
            self
        }
    }
    impl From<HtmlTableRowElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTableRowElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTableRowElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTableRowElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTableRowElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTableRowElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTableRowElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTableRowElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTableRowElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTableRowElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTableRowElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableRowElement> for Element {
    #[inline]
    fn from(obj: HtmlTableRowElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTableRowElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableRowElement> for Node {
    #[inline]
    fn from(obj: HtmlTableRowElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTableRowElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableRowElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTableRowElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTableRowElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableRowElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTableRowElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTableRowElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_cell_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `deleteCell()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn delete_cell(&self, index: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_cell_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_cell_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_delete_cell_HTMLTableRowElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_cell_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertCell()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn insert_cell(&self) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_cell_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_cell_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_insert_cell_HTMLTableRowElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_insert_cell_with_index_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <HtmlElement as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `insertCell()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn insert_cell_with_index(
        &self,
        index: i32,
    ) -> Result<HtmlElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_insert_cell_with_index_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_insert_cell_with_index_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_insert_cell_with_index_HTMLTableRowElement(self_, index)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_index_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `rowIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn row_index(&self) -> i32 {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_index_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_index_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_row_index_HTMLTableRowElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_section_row_index_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `sectionRowIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn section_row_index(&self) -> i32 {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_section_row_index_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_section_row_index_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_section_row_index_HTMLTableRowElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cells_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `cells` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn cells(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cells_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cells_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cells_HTMLTableRowElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLTableRowElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLTableRowElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn ch(&self) -> String {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_HTMLTableRowElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch(&self, ch: &str) {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch);
                __widl_f_set_ch_HTMLTableRowElement(self_, ch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_off_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn ch_off(&self) -> String {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_off_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_off_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_off_HTMLTableRowElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_off_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch_off(&self, ch_off: &str) {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_off_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch_off: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_off_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch_off = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch_off);
                __widl_f_set_ch_off_HTMLTableRowElement(self_, ch_off)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_v_align_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn v_align(&self) -> String {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_v_align_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_v_align_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_v_align_HTMLTableRowElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_v_align_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn set_v_align(&self, v_align: &str) {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_v_align_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                v_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_v_align_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let v_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(v_align);
                __widl_f_set_v_align_HTMLTableRowElement(self_, v_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bg_color_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn bg_color(&self) -> String {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bg_color_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bg_color_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bg_color_HTMLTableRowElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableRowElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_bg_color_HTMLTableRowElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableRowElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableRowElement {
    #[cfg(all(feature = "HtmlTableRowElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    #[allow(clippy::all)]
    pub fn set_bg_color(&self, bg_color: &str) {
        #[cfg(all(feature = "HtmlTableRowElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_bg_color_HTMLTableRowElement(
                self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_bg_color_HTMLTableRowElement(
            self_: <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableRowElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_set_bg_color_HTMLTableRowElement(self_, bg_color)
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
pub static __WASM_BINDGEN_GENERATED_0ec881dfb7a92a9d: [u8; 1666usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}@\x06\0\0\0\0\x11\0\0\x02\x13HTMLTableRowElement%__widl_instanceof_HTMLTableRowElement\0\0\0\0(__widl_f_delete_cell_HTMLTableRowElement\x01\0\0\x01\x13HTMLTableRowElement\x01\0\0\x01\x02\x05self_\x05index\ndeleteCell\0\0\0(__widl_f_insert_cell_HTMLTableRowElement\x01\0\0\x01\x13HTMLTableRowElement\x01\0\0\x01\x01\x05self_\ninsertCell\0\0\03__widl_f_insert_cell_with_index_HTMLTableRowElement\x01\0\0\x01\x13HTMLTableRowElement\x01\0\0\x01\x02\x05self_\x05index\ninsertCell\0\0\0&__widl_f_row_index_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x08rowIndex\x01\x01\x05self_\x08rowIndex\0\0\0.__widl_f_section_row_index_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x0FsectionRowIndex\x01\x01\x05self_\x0FsectionRowIndex\0\0\0\"__widl_f_cells_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x05cells\x01\x01\x05self_\x05cells\0\0\0\"__widl_f_align_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0&__widl_f_set_align_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\x1F__widl_f_ch_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x02ch\x01\x01\x05self_\x02ch\0\0\0#__widl_f_set_ch_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x02\x02ch\x01\x02\x05self_\x02ch\x02ch\0\0\0#__widl_f_ch_off_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x05chOff\x01\x01\x05self_\x05chOff\0\0\0'__widl_f_set_ch_off_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x02\x05chOff\x01\x02\x05self_\x06ch_off\x05chOff\0\0\0$__widl_f_v_align_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x06vAlign\x01\x01\x05self_\x06vAlign\0\0\0(__widl_f_set_v_align_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x02\x06vAlign\x01\x02\x05self_\x07v_align\x06vAlign\0\0\0%__widl_f_bg_color_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x01\x07bgColor\x01\x01\x05self_\x07bgColor\0\0\0)__widl_f_set_bg_color_HTMLTableRowElement\0\0\0\x01\x13HTMLTableRowElement\x01\0\x02\x07bgColor\x01\x02\x05self_\x08bg_color\x07bgColor\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
