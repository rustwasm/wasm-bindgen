use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGTextPositioningElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement)\n\n*This API requires the following crate features to be activated: `SvgTextPositioningElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgTextPositioningElement {
    obj: SvgTextContentElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgTextPositioningElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgTextPositioningElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(80u32);
            inform(111u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(105u32);
            inform(110u32);
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
    impl core::ops::Deref for SvgTextPositioningElement {
        type Target = SvgTextContentElement;
        #[inline]
        fn deref(&self) -> &SvgTextContentElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgTextPositioningElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgTextPositioningElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgTextPositioningElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgTextPositioningElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgTextPositioningElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgTextPositioningElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgTextPositioningElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgTextPositioningElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgTextPositioningElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgTextPositioningElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgTextPositioningElement {
        #[inline]
        fn from(obj: JsValue) -> SvgTextPositioningElement {
            SvgTextPositioningElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgTextPositioningElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgTextPositioningElement> for SvgTextPositioningElement {
        #[inline]
        fn as_ref(&self) -> &SvgTextPositioningElement {
            self
        }
    }
    impl From<SvgTextPositioningElement> for JsValue {
        #[inline]
        fn from(obj: SvgTextPositioningElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgTextPositioningElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGTextPositioningElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGTextPositioningElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGTextPositioningElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgTextPositioningElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgTextPositioningElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for SvgTextContentElement {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgTextContentElement> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &SvgTextContentElement {
        use wasm_bindgen::JsCast;
        SvgTextContentElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for SvgElement {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for Element {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for Node {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for EventTarget {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgTextPositioningElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgTextPositioningElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgTextPositioningElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SvgAnimatedLengthList",
    feature = "SvgTextPositioningElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_SVGTextPositioningElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPositioningElement as WasmDescribe>::describe();
    <SvgAnimatedLengthList as WasmDescribe>::describe();
}
impl SvgTextPositioningElement {
    #[cfg(all(
        feature = "SvgAnimatedLengthList",
        feature = "SvgTextPositioningElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> SvgAnimatedLengthList {
        #[cfg(all(
            feature = "SvgAnimatedLengthList",
            feature = "SvgTextPositioningElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_SVGTextPositioningElement(
                self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_SVGTextPositioningElement(
            self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_x_SVGTextPositioningElement(self_)
            };
            <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLengthList",
    feature = "SvgTextPositioningElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_SVGTextPositioningElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPositioningElement as WasmDescribe>::describe();
    <SvgAnimatedLengthList as WasmDescribe>::describe();
}
impl SvgTextPositioningElement {
    #[cfg(all(
        feature = "SvgAnimatedLengthList",
        feature = "SvgTextPositioningElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> SvgAnimatedLengthList {
        #[cfg(all(
            feature = "SvgAnimatedLengthList",
            feature = "SvgTextPositioningElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_SVGTextPositioningElement(
                self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_SVGTextPositioningElement(
            self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_y_SVGTextPositioningElement(self_)
            };
            <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLengthList",
    feature = "SvgTextPositioningElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dx_SVGTextPositioningElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPositioningElement as WasmDescribe>::describe();
    <SvgAnimatedLengthList as WasmDescribe>::describe();
}
impl SvgTextPositioningElement {
    #[cfg(all(
        feature = "SvgAnimatedLengthList",
        feature = "SvgTextPositioningElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `dx` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    #[allow(clippy::all)]
    pub fn dx(&self) -> SvgAnimatedLengthList {
        #[cfg(all(
            feature = "SvgAnimatedLengthList",
            feature = "SvgTextPositioningElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dx_SVGTextPositioningElement(
                self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dx_SVGTextPositioningElement(
            self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dx_SVGTextPositioningElement(self_)
            };
            <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedLengthList",
    feature = "SvgTextPositioningElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dy_SVGTextPositioningElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPositioningElement as WasmDescribe>::describe();
    <SvgAnimatedLengthList as WasmDescribe>::describe();
}
impl SvgTextPositioningElement {
    #[cfg(all(
        feature = "SvgAnimatedLengthList",
        feature = "SvgTextPositioningElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `dy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    #[allow(clippy::all)]
    pub fn dy(&self) -> SvgAnimatedLengthList {
        #[cfg(all(
            feature = "SvgAnimatedLengthList",
            feature = "SvgTextPositioningElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dy_SVGTextPositioningElement(
                self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dy_SVGTextPositioningElement(
            self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dy_SVGTextPositioningElement(self_)
            };
            <SvgAnimatedLengthList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "SvgAnimatedNumberList",
    feature = "SvgTextPositioningElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_SVGTextPositioningElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgTextPositioningElement as WasmDescribe>::describe();
    <SvgAnimatedNumberList as WasmDescribe>::describe();
}
impl SvgTextPositioningElement {
    #[cfg(all(
        feature = "SvgAnimatedNumberList",
        feature = "SvgTextPositioningElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `rotate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/rotate)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgTextPositioningElement`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self) -> SvgAnimatedNumberList {
        #[cfg(all(
            feature = "SvgAnimatedNumberList",
            feature = "SvgTextPositioningElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_SVGTextPositioningElement(
                self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_SVGTextPositioningElement(
            self_: <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgTextPositioningElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_rotate_SVGTextPositioningElement(self_)
            };
            <SvgAnimatedNumberList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_672f171893ed167f: [u8; 625usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}/\x02\0\0\0\0\x06\0\0\x02\x19SVGTextPositioningElement+__widl_instanceof_SVGTextPositioningElement\0\0\0\0$__widl_f_x_SVGTextPositioningElement\0\0\0\x01\x19SVGTextPositioningElement\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0$__widl_f_y_SVGTextPositioningElement\0\0\0\x01\x19SVGTextPositioningElement\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0%__widl_f_dx_SVGTextPositioningElement\0\0\0\x01\x19SVGTextPositioningElement\x01\0\x01\x02dx\x01\x01\x05self_\x02dx\0\0\0%__widl_f_dy_SVGTextPositioningElement\0\0\0\x01\x19SVGTextPositioningElement\x01\0\x01\x02dy\x01\x01\x05self_\x02dy\0\0\0)__widl_f_rotate_SVGTextPositioningElement\0\0\0\x01\x19SVGTextPositioningElement\x01\0\x01\x06rotate\x01\x01\x05self_\x06rotate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
