use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VREyeParameters` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters)\n\n*This API requires the following crate features to be activated: `VrEyeParameters`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrEyeParameters {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrEyeParameters: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrEyeParameters {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(86u32);
            inform(82u32);
            inform(69u32);
            inform(121u32);
            inform(101u32);
            inform(80u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for VrEyeParameters {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrEyeParameters {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrEyeParameters {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrEyeParameters {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrEyeParameters {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrEyeParameters {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrEyeParameters {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrEyeParameters {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrEyeParameters {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrEyeParameters>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrEyeParameters {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrEyeParameters {
        #[inline]
        fn from(obj: JsValue) -> VrEyeParameters {
            VrEyeParameters { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrEyeParameters {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrEyeParameters> for VrEyeParameters {
        #[inline]
        fn as_ref(&self) -> &VrEyeParameters {
            self
        }
    }
    impl From<VrEyeParameters> for JsValue {
        #[inline]
        fn from(obj: VrEyeParameters) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrEyeParameters {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VREyeParameters(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VREyeParameters(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VREyeParameters(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrEyeParameters { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrEyeParameters) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrEyeParameters> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrEyeParameters) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrEyeParameters {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrEyeParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_VREyeParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrEyeParameters as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrEyeParameters {
    #[cfg(all(feature = "VrEyeParameters",))]
    #[allow(bad_style)]
    #[doc = "The `offset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/offset)\n\n*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    #[allow(clippy::all)]
    pub fn offset(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrEyeParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_VREyeParameters(
                self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_VREyeParameters(
            self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_VREyeParameters(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrEyeParameters", feature = "VrFieldOfView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_field_of_view_VREyeParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrEyeParameters as WasmDescribe>::describe();
    <VrFieldOfView as WasmDescribe>::describe();
}
impl VrEyeParameters {
    #[cfg(all(feature = "VrEyeParameters", feature = "VrFieldOfView",))]
    #[allow(bad_style)]
    #[doc = "The `fieldOfView` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/fieldOfView)\n\n*This API requires the following crate features to be activated: `VrEyeParameters`, `VrFieldOfView`*"]
    #[allow(clippy::all)]
    pub fn field_of_view(&self) -> VrFieldOfView {
        #[cfg(all(feature = "VrEyeParameters", feature = "VrFieldOfView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_field_of_view_VREyeParameters(
                self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrFieldOfView as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_field_of_view_VREyeParameters(
            self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrFieldOfView as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_field_of_view_VREyeParameters(self_)
            };
            <VrFieldOfView as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrEyeParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_render_width_VREyeParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrEyeParameters as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VrEyeParameters {
    #[cfg(all(feature = "VrEyeParameters",))]
    #[allow(bad_style)]
    #[doc = "The `renderWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderWidth)\n\n*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    #[allow(clippy::all)]
    pub fn render_width(&self) -> u32 {
        #[cfg(all(feature = "VrEyeParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_render_width_VREyeParameters(
                self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_render_width_VREyeParameters(
            self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_render_width_VREyeParameters(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrEyeParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_render_height_VREyeParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrEyeParameters as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VrEyeParameters {
    #[cfg(all(feature = "VrEyeParameters",))]
    #[allow(bad_style)]
    #[doc = "The `renderHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderHeight)\n\n*This API requires the following crate features to be activated: `VrEyeParameters`*"]
    #[allow(clippy::all)]
    pub fn render_height(&self) -> u32 {
        #[cfg(all(feature = "VrEyeParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_render_height_VREyeParameters(
                self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_render_height_VREyeParameters(
            self_: <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrEyeParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_render_height_VREyeParameters(self_)
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
pub static __WASM_BINDGEN_GENERATED_da73ce2af150d140: [u8; 531usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD1\x01\0\0\0\0\x05\0\0\x02\x0FVREyeParameters!__widl_instanceof_VREyeParameters\0\0\0\0\x1F__widl_f_offset_VREyeParameters\x01\0\0\x01\x0FVREyeParameters\x01\0\x01\x06offset\x01\x01\x05self_\x06offset\0\0\0&__widl_f_field_of_view_VREyeParameters\0\0\0\x01\x0FVREyeParameters\x01\0\x01\x0BfieldOfView\x01\x01\x05self_\x0BfieldOfView\0\0\0%__widl_f_render_width_VREyeParameters\0\0\0\x01\x0FVREyeParameters\x01\0\x01\x0BrenderWidth\x01\x01\x05self_\x0BrenderWidth\0\0\0&__widl_f_render_height_VREyeParameters\0\0\0\x01\x0FVREyeParameters\x01\0\x01\x0CrenderHeight\x01\x01\x05self_\x0CrenderHeight\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
