use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTableCellElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTableCellElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTableCellElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTableCellElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
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
            inform(101u32);
            inform(108u32);
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
    impl core::ops::Deref for HtmlTableCellElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTableCellElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTableCellElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTableCellElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTableCellElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTableCellElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTableCellElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTableCellElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTableCellElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTableCellElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTableCellElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTableCellElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTableCellElement {
            HtmlTableCellElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTableCellElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTableCellElement> for HtmlTableCellElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTableCellElement {
            self
        }
    }
    impl From<HtmlTableCellElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTableCellElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTableCellElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTableCellElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTableCellElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTableCellElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTableCellElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTableCellElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTableCellElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTableCellElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTableCellElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableCellElement> for Element {
    #[inline]
    fn from(obj: HtmlTableCellElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTableCellElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableCellElement> for Node {
    #[inline]
    fn from(obj: HtmlTableCellElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTableCellElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableCellElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTableCellElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTableCellElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTableCellElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTableCellElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTableCellElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_col_span_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `colSpan` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn col_span(&self) -> u32 {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_col_span_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_col_span_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_col_span_HTMLTableCellElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_col_span_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `colSpan` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_col_span(&self, col_span: u32) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_col_span_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                col_span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_col_span_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            col_span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(col_span);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let col_span = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(col_span);
                __widl_f_set_col_span_HTMLTableCellElement(self_, col_span)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_row_span_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `rowSpan` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn row_span(&self) -> u32 {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_row_span_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_row_span_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_row_span_HTMLTableCellElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_row_span_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `rowSpan` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_row_span(&self, row_span: u32) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_row_span_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                row_span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_row_span_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            row_span: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(row_span);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let row_span = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(row_span);
                __widl_f_set_row_span_HTMLTableCellElement(self_, row_span)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_headers_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `headers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn headers(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_headers_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_headers_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_headers_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_headers_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `headers` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_headers(&self, headers: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_headers_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                headers: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_headers_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            headers: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(headers);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let headers = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(headers);
                __widl_f_set_headers_HTMLTableCellElement(self_, headers)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cell_index_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `cellIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn cell_index(&self) -> i32 {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cell_index_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cell_index_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cell_index_HTMLTableCellElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLTableCellElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_axis_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `axis` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn axis(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_axis_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_axis_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_axis_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_axis_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `axis` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_axis(&self, axis: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_axis_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                axis: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_axis_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            axis: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(axis);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let axis = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(axis);
                __widl_f_set_axis_HTMLTableCellElement(self_, axis)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLTableCellElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLTableCellElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn ch(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `ch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch(&self, ch: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch);
                __widl_f_set_ch_HTMLTableCellElement(self_, ch)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ch_off_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn ch_off(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ch_off_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ch_off_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ch_off_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ch_off_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `chOff` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_ch_off(&self, ch_off: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ch_off_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ch_off: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ch_off_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ch_off = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ch_off);
                __widl_f_set_ch_off_HTMLTableCellElement(self_, ch_off)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_wrap_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `noWrap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn no_wrap(&self) -> bool {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_wrap_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_wrap_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_wrap_HTMLTableCellElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_wrap_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `noWrap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_wrap(&self, no_wrap: bool) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_wrap_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_wrap: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_wrap_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_wrap: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_wrap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_wrap = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_wrap);
                __widl_f_set_no_wrap_HTMLTableCellElement(self_, no_wrap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_v_align_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn v_align(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_v_align_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_v_align_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_v_align_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_v_align_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `vAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_v_align(&self, v_align: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_v_align_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                v_align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_v_align_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let v_align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(v_align);
                __widl_f_set_v_align_HTMLTableCellElement(self_, v_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bg_color_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn bg_color(&self) -> String {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bg_color_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bg_color_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bg_color_HTMLTableCellElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTableCellElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_bg_color_HTMLTableCellElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTableCellElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTableCellElement {
    #[cfg(all(feature = "HtmlTableCellElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    #[allow(clippy::all)]
    pub fn set_bg_color(&self, bg_color: &str) {
        #[cfg(all(feature = "HtmlTableCellElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_bg_color_HTMLTableCellElement(
                self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_bg_color_HTMLTableCellElement(
            self_: <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTableCellElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_set_bg_color_HTMLTableCellElement(self_, bg_color)
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
pub static __WASM_BINDGEN_GENERATED_5131ce30961bb3c6: [u8; 2544usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAE\t\0\0\0\0\x1A\0\0\x02\x14HTMLTableCellElement&__widl_instanceof_HTMLTableCellElement\0\0\0\0&__widl_f_col_span_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x07colSpan\x01\x01\x05self_\x07colSpan\0\0\0*__widl_f_set_col_span_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x07colSpan\x01\x02\x05self_\x08col_span\x07colSpan\0\0\0&__widl_f_row_span_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x07rowSpan\x01\x01\x05self_\x07rowSpan\0\0\0*__widl_f_set_row_span_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x07rowSpan\x01\x02\x05self_\x08row_span\x07rowSpan\0\0\0%__widl_f_headers_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x07headers\x01\x01\x05self_\x07headers\0\0\0)__widl_f_set_headers_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x07headers\x01\x02\x05self_\x07headers\x07headers\0\0\0(__widl_f_cell_index_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\tcellIndex\x01\x01\x05self_\tcellIndex\0\0\0#__widl_f_align_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0'__widl_f_set_align_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\"__widl_f_axis_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x04axis\x01\x01\x05self_\x04axis\0\0\0&__widl_f_set_axis_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x04axis\x01\x02\x05self_\x04axis\x04axis\0\0\0$__widl_f_height_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0(__widl_f_set_height_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0#__widl_f_width_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0'__widl_f_set_width_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0 __widl_f_ch_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x02ch\x01\x01\x05self_\x02ch\0\0\0$__widl_f_set_ch_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x02ch\x01\x02\x05self_\x02ch\x02ch\0\0\0$__widl_f_ch_off_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x05chOff\x01\x01\x05self_\x05chOff\0\0\0(__widl_f_set_ch_off_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x05chOff\x01\x02\x05self_\x06ch_off\x05chOff\0\0\0%__widl_f_no_wrap_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x06noWrap\x01\x01\x05self_\x06noWrap\0\0\0)__widl_f_set_no_wrap_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x06noWrap\x01\x02\x05self_\x07no_wrap\x06noWrap\0\0\0%__widl_f_v_align_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x06vAlign\x01\x01\x05self_\x06vAlign\0\0\0)__widl_f_set_v_align_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x06vAlign\x01\x02\x05self_\x07v_align\x06vAlign\0\0\0&__widl_f_bg_color_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x01\x07bgColor\x01\x01\x05self_\x07bgColor\0\0\0*__widl_f_set_bg_color_HTMLTableCellElement\0\0\0\x01\x14HTMLTableCellElement\x01\0\x02\x07bgColor\x01\x02\x05self_\x08bg_color\x07bgColor\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
