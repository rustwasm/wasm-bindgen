use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CaretPosition` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)\n\n*This API requires the following crate features to be activated: `CaretPosition`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CaretPosition {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CaretPosition: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CaretPosition {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(97u32);
            inform(114u32);
            inform(101u32);
            inform(116u32);
            inform(80u32);
            inform(111u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CaretPosition {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CaretPosition {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CaretPosition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CaretPosition {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CaretPosition {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CaretPosition {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CaretPosition {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CaretPosition {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CaretPosition {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CaretPosition>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CaretPosition {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CaretPosition {
        #[inline]
        fn from(obj: JsValue) -> CaretPosition {
            CaretPosition { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CaretPosition {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CaretPosition> for CaretPosition {
        #[inline]
        fn as_ref(&self) -> &CaretPosition {
            self
        }
    }
    impl From<CaretPosition> for JsValue {
        #[inline]
        fn from(obj: CaretPosition) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CaretPosition {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CaretPosition(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CaretPosition(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CaretPosition(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CaretPosition { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CaretPosition) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CaretPosition> for ::js_sys::Object {
    #[inline]
    fn from(obj: CaretPosition) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CaretPosition {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CaretPosition", feature = "DomRect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_client_rect_CaretPosition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CaretPosition as WasmDescribe>::describe();
    <Option<DomRect> as WasmDescribe>::describe();
}
impl CaretPosition {
    #[cfg(all(feature = "CaretPosition", feature = "DomRect",))]
    #[allow(bad_style)]
    #[doc = "The `getClientRect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)\n\n*This API requires the following crate features to be activated: `CaretPosition`, `DomRect`*"]
    #[allow(clippy::all)]
    pub fn get_client_rect(&self) -> Option<DomRect> {
        #[cfg(all(feature = "CaretPosition", feature = "DomRect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_client_rect_CaretPosition(
                self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomRect> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_client_rect_CaretPosition(
            self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomRect> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_client_rect_CaretPosition(self_)
            };
            <Option<DomRect> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CaretPosition", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_node_CaretPosition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CaretPosition as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl CaretPosition {
    #[cfg(all(feature = "CaretPosition", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `offsetNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)\n\n*This API requires the following crate features to be activated: `CaretPosition`, `Node`*"]
    #[allow(clippy::all)]
    pub fn offset_node(&self) -> Option<Node> {
        #[cfg(all(feature = "CaretPosition", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_node_CaretPosition(
                self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_node_CaretPosition(
            self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_node_CaretPosition(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CaretPosition",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_CaretPosition() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CaretPosition as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl CaretPosition {
    #[cfg(all(feature = "CaretPosition",))]
    #[allow(bad_style)]
    #[doc = "The `offset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)\n\n*This API requires the following crate features to be activated: `CaretPosition`*"]
    #[allow(clippy::all)]
    pub fn offset(&self) -> u32 {
        #[cfg(all(feature = "CaretPosition",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_CaretPosition(
                self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_CaretPosition(
            self_: <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CaretPosition as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_CaretPosition(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1e61777d56e93511: [u8; 405usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}S\x01\0\0\0\0\x04\0\0\x02\rCaretPosition\x1F__widl_instanceof_CaretPosition\0\0\0\0&__widl_f_get_client_rect_CaretPosition\0\0\0\x01\rCaretPosition\x01\0\0\x01\x01\x05self_\rgetClientRect\0\0\0\"__widl_f_offset_node_CaretPosition\0\0\0\x01\rCaretPosition\x01\0\x01\noffsetNode\x01\x01\x05self_\noffsetNode\0\0\0\x1D__widl_f_offset_CaretPosition\0\0\0\x01\rCaretPosition\x01\0\x01\x06offset\x01\x01\x05self_\x06offset\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
