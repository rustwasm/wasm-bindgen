use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRFrameData` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrFrameData {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrFrameData: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrFrameData {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(86u32);
            inform(82u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    impl core::ops::Deref for VrFrameData {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrFrameData {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrFrameData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrFrameData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrFrameData {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrFrameData {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrFrameData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrFrameData {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrFrameData {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrFrameData>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrFrameData {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrFrameData {
        #[inline]
        fn from(obj: JsValue) -> VrFrameData {
            VrFrameData { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrFrameData {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrFrameData> for VrFrameData {
        #[inline]
        fn as_ref(&self) -> &VrFrameData {
            self
        }
    }
    impl From<VrFrameData> for JsValue {
        #[inline]
        fn from(obj: VrFrameData) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrFrameData {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRFrameData(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRFrameData(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRFrameData(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrFrameData { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrFrameData) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrFrameData> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrFrameData) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrFrameData {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <VrFrameData as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `new VRFrameData(..)` constructor, creating a new instance of `VRFrameData`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/VRFrameData)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<VrFrameData, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_VRFrameData() -> <VrFrameData as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_VRFrameData(
        ) -> <VrFrameData as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_VRFrameData() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<VrFrameData as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timestamp_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `timestamp` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/timestamp)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn timestamp(&self) -> f64 {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timestamp_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timestamp_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timestamp_VRFrameData(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_projection_matrix_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `leftProjectionMatrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftProjectionMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn left_projection_matrix(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_projection_matrix_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_projection_matrix_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_projection_matrix_VRFrameData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_view_matrix_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `leftViewMatrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftViewMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn left_view_matrix(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_view_matrix_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_view_matrix_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_view_matrix_VRFrameData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_right_projection_matrix_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `rightProjectionMatrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightProjectionMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn right_projection_matrix(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_right_projection_matrix_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_right_projection_matrix_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_right_projection_matrix_VRFrameData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_right_view_matrix_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `rightViewMatrix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightViewMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn right_view_matrix(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_right_view_matrix_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_right_view_matrix_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_right_view_matrix_VRFrameData(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrFrameData", feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pose_VRFrameData() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFrameData as WasmDescribe>::describe();
    <VrPose as WasmDescribe>::describe();
}
impl VrFrameData {
    #[cfg(all(feature = "VrFrameData", feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `pose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/pose)\n\n*This API requires the following crate features to be activated: `VrFrameData`, `VrPose`*"]
    #[allow(clippy::all)]
    pub fn pose(&self) -> VrPose {
        #[cfg(all(feature = "VrFrameData", feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pose_VRFrameData(
                self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrPose as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pose_VRFrameData(
            self_: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrPose as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pose_VRFrameData(self_)
            };
            <VrPose as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6e2e8da43b06548e: [u8; 783usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCD\x02\0\0\0\0\x08\0\0\x02\x0BVRFrameData\x1D__widl_instanceof_VRFrameData\0\0\0\0\x18__widl_f_new_VRFrameData\x01\0\0\x01\x0BVRFrameData\0\x01\0\x03new\0\0\0\x1E__widl_f_timestamp_VRFrameData\0\0\0\x01\x0BVRFrameData\x01\0\x01\ttimestamp\x01\x01\x05self_\ttimestamp\0\0\0+__widl_f_left_projection_matrix_VRFrameData\x01\0\0\x01\x0BVRFrameData\x01\0\x01\x14leftProjectionMatrix\x01\x01\x05self_\x14leftProjectionMatrix\0\0\0%__widl_f_left_view_matrix_VRFrameData\x01\0\0\x01\x0BVRFrameData\x01\0\x01\x0EleftViewMatrix\x01\x01\x05self_\x0EleftViewMatrix\0\0\0,__widl_f_right_projection_matrix_VRFrameData\x01\0\0\x01\x0BVRFrameData\x01\0\x01\x15rightProjectionMatrix\x01\x01\x05self_\x15rightProjectionMatrix\0\0\0&__widl_f_right_view_matrix_VRFrameData\x01\0\0\x01\x0BVRFrameData\x01\0\x01\x0FrightViewMatrix\x01\x01\x05self_\x0FrightViewMatrix\0\0\0\x19__widl_f_pose_VRFrameData\0\0\0\x01\x0BVRFrameData\x01\0\x01\x04pose\x01\x01\x05self_\x04pose\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
