use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMQuad` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomQuad {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomQuad: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomQuad {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(81u32);
            inform(117u32);
            inform(97u32);
            inform(100u32);
        }
    }
    impl core::ops::Deref for DomQuad {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomQuad {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomQuad {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomQuad {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomQuad {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomQuad {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomQuad {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomQuad {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomQuad {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomQuad>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomQuad {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomQuad {
        #[inline]
        fn from(obj: JsValue) -> DomQuad {
            DomQuad { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomQuad {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomQuad> for DomQuad {
        #[inline]
        fn as_ref(&self) -> &DomQuad {
            self
        }
    }
    impl From<DomQuad> for JsValue {
        #[inline]
        fn from(obj: DomQuad) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomQuad {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMQuad(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMQuad(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMQuad(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomQuad { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomQuad) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomQuad> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomQuad) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomQuad {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMQuad() -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMQuad() -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMQuad() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_dom_point_init_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomPointInit as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn new_with_dom_point_init(p1: &DomPointInit) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_dom_point_init_DOMQuad(
                p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_dom_point_init_DOMQuad(
            p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(p1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let p1 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p1);
                __widl_f_new_with_dom_point_init_DOMQuad(p1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_dom_point_init_and_p2_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn new_with_dom_point_init_and_p2(
        p1: &DomPointInit,
        p2: &DomPointInit,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_dom_point_init_and_p2_DOMQuad(
                p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_dom_point_init_and_p2_DOMQuad(
            p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(p1);
            drop(p2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let p1 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p1);
                let p2 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p2);
                __widl_f_new_with_dom_point_init_and_p2_DOMQuad(p1, p2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_dom_point_init_and_p2_and_p3_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn new_with_dom_point_init_and_p2_and_p3(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_dom_point_init_and_p2_and_p3_DOMQuad(
                p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p3: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_dom_point_init_and_p2_and_p3_DOMQuad(
            p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p3: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(p1);
            drop(p2);
            drop(p3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let p1 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p1);
                let p2 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p2);
                let p3 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p3);
                __widl_f_new_with_dom_point_init_and_p2_and_p3_DOMQuad(p1, p2, p3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_dom_point_init_and_p2_and_p3_and_p4_DOMQuad(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn new_with_dom_point_init_and_p2_and_p3_and_p4(
        p1: &DomPointInit,
        p2: &DomPointInit,
        p3: &DomPointInit,
        p4: &DomPointInit,
    ) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomPointInit", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_dom_point_init_and_p2_and_p3_and_p4_DOMQuad(
                p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p3: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p4: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_dom_point_init_and_p2_and_p3_and_p4_DOMQuad(
            p1: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p2: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p3: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p4: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(p1);
            drop(p2);
            drop(p3);
            drop(p4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let p1 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p1);
                let p2 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p2);
                let p3 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p3);
                let p4 = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p4);
                __widl_f_new_with_dom_point_init_and_p2_and_p3_and_p4_DOMQuad(p1, p2, p3, p4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_rect_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRectReadOnly as WasmDescribe>::describe();
    <DomQuad as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMQuad(..)` constructor, creating a new instance of `DOMQuad`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_rect(rect: &DomRectReadOnly) -> Result<DomQuad, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_rect_DOMQuad(
                rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_rect_DOMQuad(
            rect: <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuad as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(rect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let rect = <&DomRectReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rect);
                __widl_f_new_with_rect_DOMQuad(rect)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomQuad as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_bounds_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `getBounds()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn get_bounds(&self) -> DomRectReadOnly {
        #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_bounds_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_bounds_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_bounds_DOMQuad(self_)
            };
            <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomQuadJson",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomQuadJson as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomQuad", feature = "DomQuadJson",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomQuadJson`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> DomQuadJson {
        #[cfg(all(feature = "DomQuad", feature = "DomQuadJson",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomQuadJson as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomQuadJson as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_DOMQuad(self_)
            };
            <DomQuadJson as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_p1_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `p1` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn p1(&self) -> DomPoint {
        #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_p1_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_p1_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_p1_DOMQuad(self_)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_p2_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `p2` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn p2(&self) -> DomPoint {
        #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_p2_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_p2_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_p2_DOMQuad(self_)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_p3_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `p3` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn p3(&self) -> DomPoint {
        #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_p3_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_p3_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_p3_DOMQuad(self_)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_p4_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
    #[allow(bad_style)]
    #[doc = "The `p4` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
    #[allow(clippy::all)]
    pub fn p4(&self) -> DomPoint {
        #[cfg(all(feature = "DomPoint", feature = "DomQuad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_p4_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_p4_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_p4_DOMQuad(self_)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bounds_DOMQuad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomQuad as WasmDescribe>::describe();
    <DomRectReadOnly as WasmDescribe>::describe();
}
impl DomQuad {
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `bounds` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/bounds)\n\n*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
    #[allow(clippy::all)]
    pub fn bounds(&self) -> DomRectReadOnly {
        #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bounds_DOMQuad(
                self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bounds_DOMQuad(
            self_: <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomQuad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bounds_DOMQuad(self_)
            };
            <DomRectReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a8d4a5cb652b8570: [u8; 962usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x80\x03\0\0\0\0\x0E\0\0\x02\x07DOMQuad\x19__widl_instanceof_DOMQuad\0\0\0\0\x14__widl_f_new_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\0\x03new\0\0\0(__widl_f_new_with_dom_point_init_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\x01\x02p1\x03new\0\0\0/__widl_f_new_with_dom_point_init_and_p2_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\x02\x02p1\x02p2\x03new\0\0\06__widl_f_new_with_dom_point_init_and_p2_and_p3_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\x03\x02p1\x02p2\x02p3\x03new\0\0\0=__widl_f_new_with_dom_point_init_and_p2_and_p3_and_p4_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\x04\x02p1\x02p2\x02p3\x02p4\x03new\0\0\0\x1E__widl_f_new_with_rect_DOMQuad\x01\0\0\x01\x07DOMQuad\0\x01\x01\x04rect\x03new\0\0\0\x1B__widl_f_get_bounds_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\0\x01\x01\x05self_\tgetBounds\0\0\0\x18__widl_f_to_json_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\x13__widl_f_p1_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\x01\x02p1\x01\x01\x05self_\x02p1\0\0\0\x13__widl_f_p2_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\x01\x02p2\x01\x01\x05self_\x02p2\0\0\0\x13__widl_f_p3_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\x01\x02p3\x01\x01\x05self_\x02p3\0\0\0\x13__widl_f_p4_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\x01\x02p4\x01\x01\x05self_\x02p4\0\0\0\x17__widl_f_bounds_DOMQuad\0\0\0\x01\x07DOMQuad\x01\0\x01\x06bounds\x01\x01\x05self_\x06bounds\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
