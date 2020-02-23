use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTextContentElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTextContentElement {
    obj: SvgGraphicsElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTextContentElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTextContentElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(110u32);
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
    impl core::ops::Deref for SvgTextContentElement {
        type Target = SvgGraphicsElement;
        #[inline]
        fn deref(&self) -> &SvgGraphicsElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTextContentElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTextContentElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTextContentElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTextContentElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTextContentElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTextContentElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTextContentElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTextContentElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTextContentElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTextContentElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTextContentElement {
        #[inline]
        fn from(obj: JsValue) -> SvgTextContentElement {
            SvgTextContentElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTextContentElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTextContentElement> for SvgTextContentElement {
        #[inline]
        fn as_ref(&self) -> &SvgTextContentElement {
            self
        }
    }
    impl From<SvgTextContentElement> for JsValue {
        #[inline]
        fn from(obj: SvgTextContentElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTextContentElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTextContentElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTextContentElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTextContentElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTextContentElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTextContentElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTextContentElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgTextContentElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextContentElement> for SvgElement {
    #[inline]
    fn from(obj: SvgTextContentElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextContentElement> for Element {
    #[inline]
    fn from(obj: SvgTextContentElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextContentElement> for Node {
    #[inline]
    fn from(obj: SvgTextContentElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextContentElement> for EventTarget {
    #[inline]
    fn from(obj: SvgTextContentElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextContentElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTextContentElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTextContentElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_char_num_at_position_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <&SvgPoint as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getCharNumAtPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_char_num_at_position(&self, point: &SvgPoint) -> i32 {
        #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_char_num_at_position_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&SvgPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_char_num_at_position_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&SvgPoint as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(point);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&SvgPoint as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                __widl_f_get_char_num_at_position_SVGTextContentElement(self_, point)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_computed_text_length_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getComputedTextLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getComputedTextLength)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_computed_text_length(&self) -> f32 {
        #[cfg(all(feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_computed_text_length_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_computed_text_length_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_computed_text_length_SVGTextContentElement(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_end_position_of_char_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgPoint as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getEndPositionOfChar()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getEndPositionOfChar)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_end_position_of_char(
        &self,
        charnum: u32,
    ) -> Result<SvgPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_end_position_of_char_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_end_position_of_char_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(charnum);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                __widl_f_get_end_position_of_char_SVGTextContentElement(self_, charnum)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgRect", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_extent_of_char_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgRect as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgRect", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getExtentOfChar()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getExtentOfChar)\n\n*This API requires the following crate features to be activated: `SvgRect`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_extent_of_char(&self, charnum: u32) -> Result<SvgRect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgRect", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_extent_of_char_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_extent_of_char_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(charnum);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                __widl_f_get_extent_of_char_SVGTextContentElement(self_, charnum)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_number_of_chars_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getNumberOfChars()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getNumberOfChars)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_number_of_chars(&self) -> i32 {
        #[cfg(all(feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_number_of_chars_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_number_of_chars_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_number_of_chars_SVGTextContentElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_rotation_of_char_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getRotationOfChar()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getRotationOfChar)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_rotation_of_char(&self, charnum: u32) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_rotation_of_char_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_rotation_of_char_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(charnum);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                __widl_f_get_rotation_of_char_SVGTextContentElement(self_, charnum)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_start_position_of_char_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <SvgPoint as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getStartPositionOfChar()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getStartPositionOfChar)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_start_position_of_char(
        &self,
        charnum: u32,
    ) -> Result<SvgPoint, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgPoint", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_start_position_of_char_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_start_position_of_char_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(charnum);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                __widl_f_get_start_position_of_char_SVGTextContentElement(self_, charnum)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SvgPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_sub_string_length_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `getSubStringLength()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getSubStringLength)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn get_sub_string_length(
        &self,
        charnum: u32,
        nchars: u32,
    ) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_sub_string_length_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nchars: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_sub_string_length_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nchars: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(charnum);
            drop(nchars);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                let nchars = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nchars);
                __widl_f_get_sub_string_length_SVGTextContentElement(self_, charnum, nchars)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_sub_string_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectSubString()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/selectSubString)\n\n*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn select_sub_string(
        &self,
        charnum: u32,
        nchars: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_sub_string_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                nchars: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_sub_string_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charnum: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            nchars: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(charnum);
            drop(nchars);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charnum = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charnum);
                let nchars = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(nchars);
                __widl_f_select_sub_string_SVGTextContentElement(self_, charnum, nchars)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_length_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <SvgAnimatedLength as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `textLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/textLength)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn text_length(&self) -> SvgAnimatedLength {
        #[cfg(all(feature = "SvgAnimatedLength", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_length_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_length_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_length_SVGTextContentElement(self_)
            };
            <SvgAnimatedLength as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextContentElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_adjust_SVGTextContentElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextContentElement as WasmDescribe>::describe();
    <SvgAnimatedEnumeration as WasmDescribe>::describe();
}
impl SvgTextContentElement {
    #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextContentElement",))]
    #[allow(bad_style)]
    #[doc = "The `lengthAdjust` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/lengthAdjust)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextContentElement`*"]
    #[allow(clippy::all)]
    pub fn length_adjust(&self) -> SvgAnimatedEnumeration {
        #[cfg(all(feature = "SvgAnimatedEnumeration", feature = "SvgTextContentElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_adjust_SVGTextContentElement(
                self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_adjust_SVGTextContentElement(
            self_: <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextContentElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_adjust_SVGTextContentElement(self_)
            };
            <SvgAnimatedEnumeration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl SvgTextContentElement {
    pub const LENGTHADJUST_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgTextContentElement {
    pub const LENGTHADJUST_SPACING: u16 = 1u64 as u16;
}
impl SvgTextContentElement {
    pub const LENGTHADJUST_SPACINGANDGLYPHS: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d39cc11c6dcbeb92: [u8; 1469usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}{\x05\0\0\0\0\x0C\0\0\x02\x15SVGTextContentElement'__widl_instanceof_SVGTextContentElement\0\0\0\07__widl_f_get_char_num_at_position_SVGTextContentElement\0\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x02\x05self_\x05point\x14getCharNumAtPosition\0\0\07__widl_f_get_computed_text_length_SVGTextContentElement\0\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x01\x05self_\x15getComputedTextLength\0\0\07__widl_f_get_end_position_of_char_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x02\x05self_\x07charnum\x14getEndPositionOfChar\0\0\01__widl_f_get_extent_of_char_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x02\x05self_\x07charnum\x0FgetExtentOfChar\0\0\02__widl_f_get_number_of_chars_SVGTextContentElement\0\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x01\x05self_\x10getNumberOfChars\0\0\03__widl_f_get_rotation_of_char_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x02\x05self_\x07charnum\x11getRotationOfChar\0\0\09__widl_f_get_start_position_of_char_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x02\x05self_\x07charnum\x16getStartPositionOfChar\0\0\04__widl_f_get_sub_string_length_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x03\x05self_\x07charnum\x06nchars\x12getSubStringLength\0\0\00__widl_f_select_sub_string_SVGTextContentElement\x01\0\0\x01\x15SVGTextContentElement\x01\0\0\x01\x03\x05self_\x07charnum\x06nchars\x0FselectSubString\0\0\0*__widl_f_text_length_SVGTextContentElement\0\0\0\x01\x15SVGTextContentElement\x01\0\x01\ntextLength\x01\x01\x05self_\ntextLength\0\0\0,__widl_f_length_adjust_SVGTextContentElement\0\0\0\x01\x15SVGTextContentElement\x01\0\x01\x0ClengthAdjust\x01\x01\x05self_\x0ClengthAdjust\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
