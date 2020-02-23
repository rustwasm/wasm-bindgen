use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CanvasGradient` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)\n\n*This API requires the following crate features to be activated: `CanvasGradient`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CanvasGradient {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CanvasGradient: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CanvasGradient {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
            inform(71u32);
            inform(114u32);
            inform(97u32);
            inform(100u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CanvasGradient {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CanvasGradient {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CanvasGradient {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CanvasGradient {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CanvasGradient {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CanvasGradient {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CanvasGradient {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CanvasGradient {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CanvasGradient {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CanvasGradient>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CanvasGradient {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CanvasGradient {
        #[inline]
        fn from(obj: JsValue) -> CanvasGradient {
            CanvasGradient { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CanvasGradient {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CanvasGradient> for CanvasGradient {
        #[inline]
        fn as_ref(&self) -> &CanvasGradient {
            self
        }
    }
    impl From<CanvasGradient> for JsValue {
        #[inline]
        fn from(obj: CanvasGradient) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CanvasGradient {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CanvasGradient(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CanvasGradient(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CanvasGradient(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CanvasGradient { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CanvasGradient) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CanvasGradient> for ::js_sys::Object {
    #[inline]
    fn from(obj: CanvasGradient) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CanvasGradient {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CanvasGradient",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_color_stop_CanvasGradient() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CanvasGradient as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasGradient {
    #[cfg(all(feature = "CanvasGradient",))]
    #[allow(bad_style)]
    #[doc = "The `addColorStop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)\n\n*This API requires the following crate features to be activated: `CanvasGradient`*"]
    #[allow(clippy::all)]
    pub fn add_color_stop(&self, offset: f32, color: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CanvasGradient",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_color_stop_CanvasGradient(
                self_: <&CanvasGradient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_color_stop_CanvasGradient(
            self_: <&CanvasGradient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            drop(color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CanvasGradient as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(color);
                __widl_f_add_color_stop_CanvasGradient(self_, offset, color)
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
pub static __WASM_BINDGEN_GENERATED_9d71d4ef93832e3b: [u8; 255usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBD\0\0\0\0\0\x02\0\0\x02\x0ECanvasGradient __widl_instanceof_CanvasGradient\0\0\0\0&__widl_f_add_color_stop_CanvasGradient\x01\0\0\x01\x0ECanvasGradient\x01\0\0\x01\x03\x05self_\x06offset\x05color\x0CaddColorStop\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
