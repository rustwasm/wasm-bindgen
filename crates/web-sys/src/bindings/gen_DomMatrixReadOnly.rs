use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMMatrixReadOnly` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomMatrixReadOnly {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomMatrixReadOnly: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomMatrixReadOnly {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(77u32);
            inform(97u32);
            inform(116u32);
            inform(114u32);
            inform(105u32);
            inform(120u32);
            inform(82u32);
            inform(101u32);
            inform(97u32);
            inform(100u32);
            inform(79u32);
            inform(110u32);
            inform(108u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for DomMatrixReadOnly {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomMatrixReadOnly {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomMatrixReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomMatrixReadOnly {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomMatrixReadOnly {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomMatrixReadOnly {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomMatrixReadOnly {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomMatrixReadOnly {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomMatrixReadOnly {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomMatrixReadOnly>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomMatrixReadOnly {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomMatrixReadOnly {
        #[inline]
        fn from(obj: JsValue) -> DomMatrixReadOnly {
            DomMatrixReadOnly { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomMatrixReadOnly {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomMatrixReadOnly> for DomMatrixReadOnly {
        #[inline]
        fn as_ref(&self) -> &DomMatrixReadOnly {
            self
        }
    }
    impl From<DomMatrixReadOnly> for JsValue {
        #[inline]
        fn from(obj: DomMatrixReadOnly) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomMatrixReadOnly {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMMatrixReadOnly(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMMatrixReadOnly(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMMatrixReadOnly(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomMatrixReadOnly { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomMatrixReadOnly) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomMatrixReadOnly> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomMatrixReadOnly) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomMatrixReadOnly {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <DomMatrixReadOnly as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrixReadOnly(..)` constructor, creating a new instance of `DOMMatrixReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<DomMatrixReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DOMMatrixReadOnly(
            ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DOMMatrixReadOnly(
        ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_DOMMatrixReadOnly() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_str_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DomMatrixReadOnly as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrixReadOnly(..)` constructor, creating a new instance of `DOMMatrixReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_str(init: &str) -> Result<DomMatrixReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_str_DOMMatrixReadOnly(
                init: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_str_DOMMatrixReadOnly(
            init: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let init = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init);
                __widl_f_new_with_str_DOMMatrixReadOnly(init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_f64_sequence_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <DomMatrixReadOnly as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `new DOMMatrixReadOnly(..)` constructor, creating a new instance of `DOMMatrixReadOnly`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn new_with_f64_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrixReadOnly, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_f64_sequence_DOMMatrixReadOnly(
                init: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_f64_sequence_DOMMatrixReadOnly(
            init: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(init);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let init =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        init,
                    );
                __widl_f_new_with_f64_sequence_DOMMatrixReadOnly(init)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DomMatrixReadOnly as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flip_x_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `flipX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipX)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn flip_x(&self) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flip_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flip_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flip_x_DOMMatrixReadOnly(self_)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flip_y_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `flipY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipY)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn flip_y(&self) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flip_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flip_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flip_y_DOMMatrixReadOnly(self_)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inverse_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `inverse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/inverse)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn inverse(&self) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inverse_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inverse_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inverse_DOMMatrixReadOnly(self_)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiply_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <&DomMatrix as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `multiply()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/multiply)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn multiply(&self, other: &DomMatrix) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiply_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiply_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            other: <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(other);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let other = <&DomMatrix as wasm_bindgen::convert::IntoWasmAbi>::into_abi(other);
                __widl_f_multiply_DOMMatrixReadOnly(self_, other)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn rotate(&self, angle: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_DOMMatrixReadOnly(self_, angle)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_with_origin_x_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn rotate_with_origin_x(&self, angle: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_with_origin_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_with_origin_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_rotate_with_origin_x_DOMMatrixReadOnly(self_, angle, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_with_origin_x_and_origin_y_DOMMatrixReadOnly()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `rotate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn rotate_with_origin_x_and_origin_y(
        &self,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_with_origin_x_and_origin_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(angle);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_rotate_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                    self_, angle, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_axis_angle_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `rotateAxisAngle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn rotate_axis_angle(&self, x: f64, y: f64, z: f64, angle: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_axis_angle_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_axis_angle_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            drop(angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let angle = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(angle);
                __widl_f_rotate_axis_angle_DOMMatrixReadOnly(self_, x, y, z, angle)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotate_from_vector_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `rotateFromVector()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn rotate_from_vector(&self, x: f64, y: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotate_from_vector_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotate_from_vector_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_rotate_from_vector_DOMMatrixReadOnly(self_, x, y)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale(&self, scale: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                __widl_f_scale_DOMMatrixReadOnly(self_, scale)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_with_origin_x_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_with_origin_x(&self, scale: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_with_origin_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_with_origin_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale_with_origin_x_DOMMatrixReadOnly(self_, scale, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_with_origin_x_and_origin_y_DOMMatrixReadOnly()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_with_origin_x_and_origin_y(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_with_origin_x_and_origin_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                    self_, scale, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale3d()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale3d(&self, scale: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                __widl_f_scale3d_DOMMatrixReadOnly(self_, scale)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_with_origin_x_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale3d()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale3d_with_origin_x(&self, scale: f64, origin_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_with_origin_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_with_origin_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale3d_with_origin_x_DOMMatrixReadOnly(self_, scale, origin_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_with_origin_x_and_origin_y_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale3d()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale3d_with_origin_x_and_origin_y(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_with_origin_x_and_origin_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale3d_with_origin_x_and_origin_y_DOMMatrixReadOnly(
                    self_, scale, origin_x, origin_y,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale3d_with_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scale3d()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale3d_with_origin_x_and_origin_y_and_origin_z(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale3d_with_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale3d_with_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale);
            drop(origin_x);
            drop(origin_y);
            drop(origin_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                let origin_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_z);
                __widl_f_scale3d_with_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
                    self_, scale, origin_x, origin_y, origin_z,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform(&self, scale_x: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                __widl_f_scale_non_uniform_DOMMatrixReadOnly(self_, scale_x)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_with_scale_y_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_with_scale_y(&self, scale_x: f64, scale_y: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_with_scale_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_with_scale_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                __widl_f_scale_non_uniform_with_scale_y_DOMMatrixReadOnly(self_, scale_x, scale_y)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_with_scale_y_and_scale_z_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_with_scale_y_and_scale_z(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                __widl_f_scale_non_uniform_with_scale_y_and_scale_z_DOMMatrixReadOnly(
                    self_, scale_x, scale_y, scale_z,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_DOMMatrixReadOnly(
                    self_, scale_x, scale_y, scale_z, origin_x,
                )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            drop(origin_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrixReadOnly ( self_ , scale_x , scale_y , scale_z , origin_x , origin_y )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `scaleNonUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scale_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            origin_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(scale_x);
            drop(scale_y);
            drop(scale_z);
            drop(origin_x);
            drop(origin_y);
            drop(origin_z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scale_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_x);
                let scale_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_y);
                let scale_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scale_z);
                let origin_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_x);
                let origin_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_y);
                let origin_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(origin_z);
                __widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly ( self_ , scale_x , scale_y , scale_z , origin_x , origin_y , origin_z )
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_x_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `skewX()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewX)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn skew_x(&self, sx: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_x_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_x_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sx);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sx);
                __widl_f_skew_x_DOMMatrixReadOnly(self_, sx)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_skew_y_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `skewY()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewY)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn skew_y(&self, sy: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_skew_y_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_skew_y_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sy: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(sy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sy = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sy);
                __widl_f_skew_y_DOMMatrixReadOnly(self_, sy)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_float32_array_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `toFloat32Array()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat32Array)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn to_float32_array(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_float32_array_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_float32_array_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_float32_array_DOMMatrixReadOnly(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_float64_array_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <Vec<f64> as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `toFloat64Array()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat64Array)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn to_float64_array(&self) -> Result<Vec<f64>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_float64_array_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_float64_array_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_float64_array_DOMMatrixReadOnly(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toJSON)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_DOMMatrixReadOnly(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly", feature = "DomPoint",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_point_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly", feature = "DomPoint",))]
    #[allow(bad_style)]
    #[doc = "The `transformPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`*"]
    #[allow(clippy::all)]
    pub fn transform_point(&self) -> DomPoint {
        #[cfg(all(feature = "DomMatrixReadOnly", feature = "DomPoint",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_point_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_point_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transform_point_DOMMatrixReadOnly(self_)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "DomMatrixReadOnly",
    feature = "DomPoint",
    feature = "DomPointInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transform_point_with_point_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <&DomPointInit as WasmDescribe>::describe();
    <DomPoint as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(
        feature = "DomMatrixReadOnly",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `transformPoint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`, `DomPointInit`*"]
    #[allow(clippy::all)]
    pub fn transform_point_with_point(&self, point: &DomPointInit) -> DomPoint {
        #[cfg(all(
            feature = "DomMatrixReadOnly",
            feature = "DomPoint",
            feature = "DomPointInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transform_point_with_point_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transform_point_with_point_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            point: <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomPoint as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let point = <&DomPointInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(point);
                __widl_f_transform_point_with_point_DOMMatrixReadOnly(self_, point)
            };
            <DomPoint as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn translate(&self, tx: f64, ty: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(tx);
            drop(ty);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                __widl_f_translate_DOMMatrixReadOnly(self_, tx, ty)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_translate_with_tz_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DomMatrix as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `translate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)\n\n*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn translate_with_tz(&self, tx: f64, ty: f64, tz: f64) -> DomMatrix {
        #[cfg(all(feature = "DomMatrix", feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_translate_with_tz_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_translate_with_tz_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tx: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ty: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tz: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(tx);
            drop(ty);
            drop(tz);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tx = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tx);
                let ty = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ty);
                let tz = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tz);
                __widl_f_translate_with_tz_DOMMatrixReadOnly(self_, tx, ty, tz)
            };
            <DomMatrix as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_a_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `a` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/a)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn a(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_a_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_a_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_a_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_b_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `b` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/b)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn b(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_b_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_b_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_b_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_c_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `c` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/c)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn c(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_c_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_c_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_c_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_d_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `d` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/d)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn d(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_d_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_d_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_d_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_e_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `e` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/e)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn e(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_e_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_e_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_e_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_f_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `f` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/f)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn f(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_f_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_f_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_f_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m11_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m11` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m11)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m11(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m11_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m11_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m11_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m12_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m12` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m12)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m12(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m12_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m12_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m12_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m13_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m13` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m13)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m13(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m13_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m13_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m13_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m14_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m14` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m14)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m14(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m14_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m14_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m14_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m21_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m21` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m21)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m21(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m21_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m21_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m21_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m22_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m22` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m22)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m22(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m22_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m22_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m22_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m23_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m23` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m23)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m23(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m23_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m23_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m23_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m24_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m24` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m24)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m24(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m24_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m24_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m24_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m31_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m31` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m31)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m31(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m31_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m31_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m31_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m32_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m32` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m32)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m32(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m32_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m32_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m32_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m33_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m33` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m33)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m33(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m33_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m33_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m33_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m34_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m34` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m34)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m34(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m34_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m34_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m34_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m41_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m41` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m41)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m41(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m41_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m41_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m41_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m42_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m42` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m42)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m42(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m42_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m42_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m42_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m43_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m43` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m43)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m43(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m43_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m43_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m43_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_m44_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `m44` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m44)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn m44(&self) -> f64 {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_m44_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_m44_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_m44_DOMMatrixReadOnly(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_2d_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `is2D` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/is2D)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn is_2d(&self) -> bool {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_2d_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_2d_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_2d_DOMMatrixReadOnly(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomMatrixReadOnly",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_identity_DOMMatrixReadOnly() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomMatrixReadOnly as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomMatrixReadOnly {
    #[cfg(all(feature = "DomMatrixReadOnly",))]
    #[allow(bad_style)]
    #[doc = "The `isIdentity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/isIdentity)\n\n*This API requires the following crate features to be activated: `DomMatrixReadOnly`*"]
    #[allow(clippy::all)]
    pub fn is_identity(&self) -> bool {
        #[cfg(all(feature = "DomMatrixReadOnly",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_identity_DOMMatrixReadOnly(
                self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_identity_DOMMatrixReadOnly(
            self_: <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DomMatrixReadOnly as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_identity_DOMMatrixReadOnly(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_83e3f7939657aa96: [u8; 5579usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x89\x15\0\0\0\0;\0\0\x02\x11DOMMatrixReadOnly#__widl_instanceof_DOMMatrixReadOnly\0\0\0\0\x1E__widl_f_new_DOMMatrixReadOnly\x01\0\0\x01\x11DOMMatrixReadOnly\0\x01\0\x03new\0\0\0'__widl_f_new_with_str_DOMMatrixReadOnly\x01\0\0\x01\x11DOMMatrixReadOnly\0\x01\x01\x04init\x03new\0\0\00__widl_f_new_with_f64_sequence_DOMMatrixReadOnly\x01\0\0\x01\x11DOMMatrixReadOnly\0\x01\x01\x04init\x03new\0\0\0!__widl_f_flip_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x05flipX\0\0\0!__widl_f_flip_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x05flipY\0\0\0\"__widl_f_inverse_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x07inverse\0\0\0#__widl_f_multiply_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x05other\x08multiply\0\0\0!__widl_f_rotate_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x05angle\x06rotate\0\0\0/__widl_f_rotate_with_origin_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x05angle\x08origin_x\x06rotate\0\0\0<__widl_f_rotate_with_origin_x_and_origin_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x04\x05self_\x05angle\x08origin_x\x08origin_y\x06rotate\0\0\0,__widl_f_rotate_axis_angle_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x05\x05self_\x01x\x01y\x01z\x05angle\x0FrotateAxisAngle\0\0\0-__widl_f_rotate_from_vector_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x01x\x01y\x10rotateFromVector\0\0\0 __widl_f_scale_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x05scale\x05scale\0\0\0.__widl_f_scale_with_origin_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x05scale\x08origin_x\x05scale\0\0\0;__widl_f_scale_with_origin_x_and_origin_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x04\x05self_\x05scale\x08origin_x\x08origin_y\x05scale\0\0\0\"__widl_f_scale3d_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x05scale\x07scale3d\0\0\00__widl_f_scale3d_with_origin_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x05scale\x08origin_x\x07scale3d\0\0\0=__widl_f_scale3d_with_origin_x_and_origin_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x04\x05self_\x05scale\x08origin_x\x08origin_y\x07scale3d\0\0\0J__widl_f_scale3d_with_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x05\x05self_\x05scale\x08origin_x\x08origin_y\x08origin_z\x07scale3d\0\0\0,__widl_f_scale_non_uniform_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x07scale_x\x0FscaleNonUniform\0\0\09__widl_f_scale_non_uniform_with_scale_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x07scale_x\x07scale_y\x0FscaleNonUniform\0\0\0E__widl_f_scale_non_uniform_with_scale_y_and_scale_z_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x04\x05self_\x07scale_x\x07scale_y\x07scale_z\x0FscaleNonUniform\0\0\0R__widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x05\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x0FscaleNonUniform\0\0\0___widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x06\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x08origin_y\x0FscaleNonUniform\0\0\0l__widl_f_scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x07\x05self_\x07scale_x\x07scale_y\x07scale_z\x08origin_x\x08origin_y\x08origin_z\x0FscaleNonUniform\0\0\0!__widl_f_skew_x_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x02sx\x05skewX\0\0\0!__widl_f_skew_y_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x02sy\x05skewY\0\0\0+__widl_f_to_float32_array_DOMMatrixReadOnly\x01\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x0EtoFloat32Array\0\0\0+__widl_f_to_float64_array_DOMMatrixReadOnly\x01\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x0EtoFloat64Array\0\0\0\"__widl_f_to_json_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0*__widl_f_transform_point_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x01\x05self_\x0EtransformPoint\0\0\05__widl_f_transform_point_with_point_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x02\x05self_\x05point\x0EtransformPoint\0\0\0$__widl_f_translate_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x03\x05self_\x02tx\x02ty\ttranslate\0\0\0,__widl_f_translate_with_tz_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\0\x01\x04\x05self_\x02tx\x02ty\x02tz\ttranslate\0\0\0\x1C__widl_f_a_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01a\x01\x01\x05self_\x01a\0\0\0\x1C__widl_f_b_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01b\x01\x01\x05self_\x01b\0\0\0\x1C__widl_f_c_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01c\x01\x01\x05self_\x01c\0\0\0\x1C__widl_f_d_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01d\x01\x01\x05self_\x01d\0\0\0\x1C__widl_f_e_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01e\x01\x01\x05self_\x01e\0\0\0\x1C__widl_f_f_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x01f\x01\x01\x05self_\x01f\0\0\0\x1E__widl_f_m11_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m11\x01\x01\x05self_\x03m11\0\0\0\x1E__widl_f_m12_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m12\x01\x01\x05self_\x03m12\0\0\0\x1E__widl_f_m13_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m13\x01\x01\x05self_\x03m13\0\0\0\x1E__widl_f_m14_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m14\x01\x01\x05self_\x03m14\0\0\0\x1E__widl_f_m21_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m21\x01\x01\x05self_\x03m21\0\0\0\x1E__widl_f_m22_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m22\x01\x01\x05self_\x03m22\0\0\0\x1E__widl_f_m23_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m23\x01\x01\x05self_\x03m23\0\0\0\x1E__widl_f_m24_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m24\x01\x01\x05self_\x03m24\0\0\0\x1E__widl_f_m31_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m31\x01\x01\x05self_\x03m31\0\0\0\x1E__widl_f_m32_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m32\x01\x01\x05self_\x03m32\0\0\0\x1E__widl_f_m33_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m33\x01\x01\x05self_\x03m33\0\0\0\x1E__widl_f_m34_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m34\x01\x01\x05self_\x03m34\0\0\0\x1E__widl_f_m41_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m41\x01\x01\x05self_\x03m41\0\0\0\x1E__widl_f_m42_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m42\x01\x01\x05self_\x03m42\0\0\0\x1E__widl_f_m43_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m43\x01\x01\x05self_\x03m43\0\0\0\x1E__widl_f_m44_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x03m44\x01\x01\x05self_\x03m44\0\0\0 __widl_f_is_2d_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\x04is2D\x01\x01\x05self_\x04is2D\0\0\0&__widl_f_is_identity_DOMMatrixReadOnly\0\0\0\x01\x11DOMMatrixReadOnly\x01\0\x01\nisIdentity\x01\x01\x05self_\nisIdentity\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
