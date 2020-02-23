use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaintWorkletGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope)\n\n*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaintWorkletGlobalScope {
    obj: WorkletGlobalScope,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaintWorkletGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaintWorkletGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(80u32);
            inform(97u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(108u32);
            inform(101u32);
            inform(116u32);
            inform(71u32);
            inform(108u32);
            inform(111u32);
            inform(98u32);
            inform(97u32);
            inform(108u32);
            inform(83u32);
            inform(99u32);
            inform(111u32);
            inform(112u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for PaintWorkletGlobalScope {
        type Target = WorkletGlobalScope;
        #[inline]
        fn deref(&self) -> &WorkletGlobalScope {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaintWorkletGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaintWorkletGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaintWorkletGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaintWorkletGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaintWorkletGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaintWorkletGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaintWorkletGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaintWorkletGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaintWorkletGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaintWorkletGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaintWorkletGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> PaintWorkletGlobalScope {
            PaintWorkletGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaintWorkletGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaintWorkletGlobalScope> for PaintWorkletGlobalScope {
        #[inline]
        fn as_ref(&self) -> &PaintWorkletGlobalScope {
            self
        }
    }
    impl From<PaintWorkletGlobalScope> for JsValue {
        #[inline]
        fn from(obj: PaintWorkletGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaintWorkletGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaintWorkletGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaintWorkletGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaintWorkletGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaintWorkletGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaintWorkletGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaintWorkletGlobalScope> for WorkletGlobalScope {
    #[inline]
    fn from(obj: PaintWorkletGlobalScope) -> WorkletGlobalScope {
        use wasm_bindgen::JsCast;
        WorkletGlobalScope::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<WorkletGlobalScope> for PaintWorkletGlobalScope {
    #[inline]
    fn as_ref(&self) -> &WorkletGlobalScope {
        use wasm_bindgen::JsCast;
        WorkletGlobalScope::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PaintWorkletGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaintWorkletGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaintWorkletGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PaintWorkletGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_paint_PaintWorkletGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&PaintWorkletGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PaintWorkletGlobalScope {
    #[cfg(all(feature = "PaintWorkletGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `registerPaint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/registerPaint)\n\n*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn register_paint(&self, name: &str, paint_ctor: &::js_sys::Function) {
        #[cfg(all(feature = "PaintWorkletGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_paint_PaintWorkletGlobalScope(
                self_: <&PaintWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                paint_ctor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_paint_PaintWorkletGlobalScope(
            self_: <&PaintWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            paint_ctor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(paint_ctor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaintWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let paint_ctor =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        paint_ctor,
                    );
                __widl_f_register_paint_PaintWorkletGlobalScope(self_, name, paint_ctor)
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
pub static __WASM_BINDGEN_GENERATED_d664f8a4f41fc5d9: [u8; 295usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE5\0\0\0\0\0\x02\0\0\x02\x17PaintWorkletGlobalScope)__widl_instanceof_PaintWorkletGlobalScope\0\0\0\0/__widl_f_register_paint_PaintWorkletGlobalScope\0\0\0\x01\x17PaintWorkletGlobalScope\x01\0\0\x01\x03\x05self_\x04name\npaint_ctor\rregisterPaint\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
