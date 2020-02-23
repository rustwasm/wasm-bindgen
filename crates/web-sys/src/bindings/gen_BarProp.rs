use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BarProp` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BarProp {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BarProp: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BarProp {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(66u32);
            inform(97u32);
            inform(114u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(112u32);
        }
    }
    impl core::ops::Deref for BarProp {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for BarProp {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BarProp {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BarProp {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BarProp {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BarProp {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BarProp {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BarProp {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BarProp {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BarProp>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BarProp {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BarProp {
        #[inline]
        fn from(obj: JsValue) -> BarProp {
            BarProp { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BarProp {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BarProp> for BarProp {
        #[inline]
        fn as_ref(&self) -> &BarProp {
            self
        }
    }
    impl From<BarProp> for JsValue {
        #[inline]
        fn from(obj: BarProp) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BarProp {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BarProp(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BarProp(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BarProp(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BarProp { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BarProp) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BarProp> for ::js_sys::Object {
    #[inline]
    fn from(obj: BarProp) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BarProp {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BarProp",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_visible_BarProp() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BarProp as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl BarProp {
    #[cfg(all(feature = "BarProp",))]
    #[allow(bad_style)]
    #[doc = "The `visible` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
    #[allow(clippy::all)]
    pub fn visible(&self) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BarProp",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_visible_BarProp(
                self_: <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_visible_BarProp(
            self_: <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_visible_BarProp(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BarProp",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_visible_BarProp() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BarProp as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BarProp {
    #[cfg(all(feature = "BarProp",))]
    #[allow(bad_style)]
    #[doc = "The `visible` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)\n\n*This API requires the following crate features to be activated: `BarProp`*"]
    #[allow(clippy::all)]
    pub fn set_visible(&self, visible: bool) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BarProp",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_visible_BarProp(
                self_: <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                visible: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_visible_BarProp(
            self_: <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            visible: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(visible);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BarProp as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let visible = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(visible);
                __widl_f_set_visible_BarProp(self_, visible)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5ddc9b7d2947d750: [u8; 289usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDF\0\0\0\0\0\x03\0\0\x02\x07BarProp\x19__widl_instanceof_BarProp\0\0\0\0\x18__widl_f_visible_BarProp\x01\0\0\x01\x07BarProp\x01\0\x01\x07visible\x01\x01\x05self_\x07visible\0\0\0\x1C__widl_f_set_visible_BarProp\x01\0\0\x01\x07BarProp\x01\0\x02\x07visible\x01\x02\x05self_\x07visible\x07visible\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
