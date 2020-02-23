use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaintRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest)\n\n*This API requires the following crate features to be activated: `PaintRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaintRequest {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaintRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaintRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(80u32);
            inform(97u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PaintRequest {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaintRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaintRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaintRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaintRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaintRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaintRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaintRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaintRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaintRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaintRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaintRequest {
        #[inline]
        fn from(obj: JsValue) -> PaintRequest {
            PaintRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaintRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaintRequest> for PaintRequest {
        #[inline]
        fn as_ref(&self) -> &PaintRequest {
            self
        }
    }
    impl From<PaintRequest> for JsValue {
        #[inline]
        fn from(obj: PaintRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaintRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaintRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaintRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaintRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaintRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaintRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaintRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaintRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaintRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomRect", feature = "PaintRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_rect_PaintRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaintRequest as WasmDescribe>::describe();
    <DomRect as WasmDescribe>::describe();
}
impl PaintRequest {
    #[cfg(all(feature = "DomRect", feature = "PaintRequest",))]
    #[allow(bad_style)]
    #[doc = "The `clientRect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/clientRect)\n\n*This API requires the following crate features to be activated: `DomRect`, `PaintRequest`*"]
    #[allow(clippy::all)]
    pub fn client_rect(&self) -> DomRect {
        #[cfg(all(feature = "DomRect", feature = "PaintRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_rect_PaintRequest(
                self_: <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_rect_PaintRequest(
            self_: <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_rect_PaintRequest(self_)
            };
            <DomRect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaintRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reason_PaintRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaintRequest as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaintRequest {
    #[cfg(all(feature = "PaintRequest",))]
    #[allow(bad_style)]
    #[doc = "The `reason` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/reason)\n\n*This API requires the following crate features to be activated: `PaintRequest`*"]
    #[allow(clippy::all)]
    pub fn reason(&self) -> String {
        #[cfg(all(feature = "PaintRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reason_PaintRequest(
                self_: <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reason_PaintRequest(
            self_: <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaintRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reason_PaintRequest(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8be5a801069ff6dd: [u8; 314usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF8\0\0\0\0\0\x03\0\0\x02\x0CPaintRequest\x1E__widl_instanceof_PaintRequest\0\0\0\0!__widl_f_client_rect_PaintRequest\0\0\0\x01\x0CPaintRequest\x01\0\x01\nclientRect\x01\x01\x05self_\nclientRect\0\0\0\x1C__widl_f_reason_PaintRequest\0\0\0\x01\x0CPaintRequest\x01\0\x01\x06reason\x01\x01\x05self_\x06reason\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
