use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ANGLE_instanced_arrays` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AngleInstancedArrays {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AngleInstancedArrays: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AngleInstancedArrays {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(65u32);
            inform(78u32);
            inform(71u32);
            inform(76u32);
            inform(69u32);
            inform(95u32);
            inform(105u32);
            inform(110u32);
            inform(115u32);
            inform(116u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
            inform(100u32);
            inform(95u32);
            inform(97u32);
            inform(114u32);
            inform(114u32);
            inform(97u32);
            inform(121u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for AngleInstancedArrays {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AngleInstancedArrays {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AngleInstancedArrays {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AngleInstancedArrays {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AngleInstancedArrays {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AngleInstancedArrays {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AngleInstancedArrays {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AngleInstancedArrays {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AngleInstancedArrays {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AngleInstancedArrays>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AngleInstancedArrays {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AngleInstancedArrays {
        #[inline]
        fn from(obj: JsValue) -> AngleInstancedArrays {
            AngleInstancedArrays { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AngleInstancedArrays {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AngleInstancedArrays> for AngleInstancedArrays {
        #[inline]
        fn as_ref(&self) -> &AngleInstancedArrays {
            self
        }
    }
    impl From<AngleInstancedArrays> for JsValue {
        #[inline]
        fn from(obj: AngleInstancedArrays) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AngleInstancedArrays {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ANGLE_instanced_arrays(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ANGLE_instanced_arrays(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ANGLE_instanced_arrays(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AngleInstancedArrays { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AngleInstancedArrays) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AngleInstancedArrays> for ::js_sys::Object {
    #[inline]
    fn from(obj: AngleInstancedArrays) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AngleInstancedArrays {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AngleInstancedArrays",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_arrays_instanced_angle_ANGLE_instanced_arrays()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&AngleInstancedArrays as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AngleInstancedArrays {
    #[cfg(all(feature = "AngleInstancedArrays",))]
    #[allow(bad_style)]
    #[doc = "The `drawArraysInstancedANGLE()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawArraysInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    #[allow(clippy::all)]
    pub fn draw_arrays_instanced_angle(&self, mode: u32, first: i32, count: i32, primcount: i32) {
        #[cfg(all(feature = "AngleInstancedArrays",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_arrays_instanced_angle_ANGLE_instanced_arrays(
                self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                first: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_arrays_instanced_angle_ANGLE_instanced_arrays(
            self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            first: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(first);
            drop(count);
            drop(primcount);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let first = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(first);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let primcount = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(primcount);
                __widl_f_draw_arrays_instanced_angle_ANGLE_instanced_arrays(
                    self_, mode, first, count, primcount,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "AngleInstancedArrays",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_elements_instanced_angle_with_i32_ANGLE_instanced_arrays(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&AngleInstancedArrays as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AngleInstancedArrays {
    #[cfg(all(feature = "AngleInstancedArrays",))]
    #[allow(bad_style)]
    #[doc = "The `drawElementsInstancedANGLE()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    #[allow(clippy::all)]
    pub fn draw_elements_instanced_angle_with_i32(
        &self,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        primcount: i32,
    ) {
        #[cfg(all(feature = "AngleInstancedArrays",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_elements_instanced_angle_with_i32_ANGLE_instanced_arrays(
                self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_elements_instanced_angle_with_i32_ANGLE_instanced_arrays(
            self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(count);
            drop(type_);
            drop(offset);
            drop(primcount);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let primcount = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(primcount);
                __widl_f_draw_elements_instanced_angle_with_i32_ANGLE_instanced_arrays(
                    self_, mode, count, type_, offset, primcount,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "AngleInstancedArrays",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_elements_instanced_angle_with_f64_ANGLE_instanced_arrays(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&AngleInstancedArrays as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AngleInstancedArrays {
    #[cfg(all(feature = "AngleInstancedArrays",))]
    #[allow(bad_style)]
    #[doc = "The `drawElementsInstancedANGLE()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    #[allow(clippy::all)]
    pub fn draw_elements_instanced_angle_with_f64(
        &self,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
        primcount: i32,
    ) {
        #[cfg(all(feature = "AngleInstancedArrays",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_elements_instanced_angle_with_f64_ANGLE_instanced_arrays(
                self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_elements_instanced_angle_with_f64_ANGLE_instanced_arrays(
            self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            primcount: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(count);
            drop(type_);
            drop(offset);
            drop(primcount);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let primcount = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(primcount);
                __widl_f_draw_elements_instanced_angle_with_f64_ANGLE_instanced_arrays(
                    self_, mode, count, type_, offset, primcount,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "AngleInstancedArrays",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib_divisor_angle_ANGLE_instanced_arrays()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AngleInstancedArrays as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AngleInstancedArrays {
    #[cfg(all(feature = "AngleInstancedArrays",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttribDivisorANGLE()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/vertexAttribDivisorANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib_divisor_angle(&self, index: u32, divisor: u32) {
        #[cfg(all(feature = "AngleInstancedArrays",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib_divisor_angle_ANGLE_instanced_arrays(
                self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                divisor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib_divisor_angle_ANGLE_instanced_arrays(
            self_: <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            divisor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(divisor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AngleInstancedArrays as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let divisor = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(divisor);
                __widl_f_vertex_attrib_divisor_angle_ANGLE_instanced_arrays(self_, index, divisor)
            };
            ()
        }
    }
}
impl AngleInstancedArrays {
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: u32 = 35070u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a90fbe416e5b8ff9: [u8; 812usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEA\x02\0\0\0\0\x05\0\0\x02\x16ANGLE_instanced_arrays(__widl_instanceof_ANGLE_instanced_arrays\0\0\0\0;__widl_f_draw_arrays_instanced_angle_ANGLE_instanced_arrays\0\0\0\x01\x16ANGLE_instanced_arrays\x01\0\0\x01\x05\x05self_\x04mode\x05first\x05count\tprimcount\x18drawArraysInstancedANGLE\0\0\0F__widl_f_draw_elements_instanced_angle_with_i32_ANGLE_instanced_arrays\0\0\0\x01\x16ANGLE_instanced_arrays\x01\0\0\x01\x06\x05self_\x04mode\x05count\x05type_\x06offset\tprimcount\x1AdrawElementsInstancedANGLE\0\0\0F__widl_f_draw_elements_instanced_angle_with_f64_ANGLE_instanced_arrays\0\0\0\x01\x16ANGLE_instanced_arrays\x01\0\0\x01\x06\x05self_\x04mode\x05count\x05type_\x06offset\tprimcount\x1AdrawElementsInstancedANGLE\0\0\0;__widl_f_vertex_attrib_divisor_angle_ANGLE_instanced_arrays\0\0\0\x01\x16ANGLE_instanced_arrays\x01\0\0\x01\x03\x05self_\x05index\x07divisor\x18vertexAttribDivisorANGLE\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
