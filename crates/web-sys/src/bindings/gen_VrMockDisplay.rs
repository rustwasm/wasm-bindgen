use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRMockDisplay` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay)\n\n*This API requires the following crate features to be activated: `VrMockDisplay`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrMockDisplay {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrMockDisplay: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrMockDisplay {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(86u32);
            inform(82u32);
            inform(77u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(68u32);
            inform(105u32);
            inform(115u32);
            inform(112u32);
            inform(108u32);
            inform(97u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for VrMockDisplay {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrMockDisplay {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrMockDisplay {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrMockDisplay {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrMockDisplay {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrMockDisplay {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrMockDisplay {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrMockDisplay {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrMockDisplay {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrMockDisplay>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrMockDisplay {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrMockDisplay {
        #[inline]
        fn from(obj: JsValue) -> VrMockDisplay {
            VrMockDisplay { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrMockDisplay {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrMockDisplay> for VrMockDisplay {
        #[inline]
        fn as_ref(&self) -> &VrMockDisplay {
            self
        }
    }
    impl From<VrMockDisplay> for JsValue {
        #[inline]
        fn from(obj: VrMockDisplay) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrMockDisplay {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRMockDisplay(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRMockDisplay(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRMockDisplay(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrMockDisplay { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrMockDisplay) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrMockDisplay> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrMockDisplay) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrMockDisplay {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrEye", feature = "VrMockDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_eye_parameter_VRMockDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&VrMockDisplay as WasmDescribe>::describe();
    <VrEye as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockDisplay {
    #[cfg(all(feature = "VrEye", feature = "VrMockDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `setEyeParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setEyeParameter)\n\n*This API requires the following crate features to be activated: `VrEye`, `VrMockDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_eye_parameter(
        &self,
        eye: VrEye,
        offset_x: f64,
        offset_y: f64,
        offset_z: f64,
        up_degree: f64,
        right_degree: f64,
        down_degree: f64,
        left_degree: f64,
    ) {
        #[cfg(all(feature = "VrEye", feature = "VrMockDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_eye_parameter_VRMockDisplay(
                self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                eye: <VrEye as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                up_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                right_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                down_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                left_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_eye_parameter_VRMockDisplay(
            self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            eye: <VrEye as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset_z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            up_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            right_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            down_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            left_degree: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(eye);
            drop(offset_x);
            drop(offset_y);
            drop(offset_z);
            drop(up_degree);
            drop(right_degree);
            drop(down_degree);
            drop(left_degree);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let eye = <VrEye as wasm_bindgen::convert::IntoWasmAbi>::into_abi(eye);
                let offset_x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset_x);
                let offset_y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset_y);
                let offset_z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset_z);
                let up_degree = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(up_degree);
                let right_degree =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(right_degree);
                let down_degree =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(down_degree);
                let left_degree =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(left_degree);
                __widl_f_set_eye_parameter_VRMockDisplay(
                    self_,
                    eye,
                    offset_x,
                    offset_y,
                    offset_z,
                    up_degree,
                    right_degree,
                    down_degree,
                    left_degree,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_eye_resolution_VRMockDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&VrMockDisplay as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockDisplay {
    #[cfg(all(feature = "VrMockDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `setEyeResolution()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setEyeResolution)\n\n*This API requires the following crate features to be activated: `VrMockDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_eye_resolution(&self, a_render_width: u32, a_render_height: u32) {
        #[cfg(all(feature = "VrMockDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_eye_resolution_VRMockDisplay(
                self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_render_width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_render_height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_eye_resolution_VRMockDisplay(
            self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_render_width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_render_height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_render_width);
            drop(a_render_height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_render_width =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_render_width);
                let a_render_height =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_render_height);
                __widl_f_set_eye_resolution_VRMockDisplay(self_, a_render_width, a_render_height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_mount_state_VRMockDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrMockDisplay as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockDisplay {
    #[cfg(all(feature = "VrMockDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `setMountState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setMountState)\n\n*This API requires the following crate features to be activated: `VrMockDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_mount_state(&self, is_mounted: bool) {
        #[cfg(all(feature = "VrMockDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_mount_state_VRMockDisplay(
                self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                is_mounted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_mount_state_VRMockDisplay(
            self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            is_mounted: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(is_mounted);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let is_mounted = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(is_mounted);
                __widl_f_set_mount_state_VRMockDisplay(self_, is_mounted)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pose_VRMockDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&VrMockDisplay as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockDisplay {
    #[cfg(all(feature = "VrMockDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `setPose()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)\n\n*This API requires the following crate features to be activated: `VrMockDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_pose(
        &self,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    ) {
        #[cfg(all(feature = "VrMockDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pose_VRMockDisplay(
                self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                position: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                linear_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                linear_acceleration : < Option < & mut [ f32 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                orientation: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angular_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angular_acceleration : < Option < & mut [ f32 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pose_VRMockDisplay(
            self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            position: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            linear_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            linear_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            orientation: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angular_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            angular_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(position);
            drop(linear_velocity);
            drop(linear_acceleration);
            drop(orientation);
            drop(angular_velocity);
            drop(angular_acceleration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let position =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(position);
                let linear_velocity =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        linear_velocity,
                    );
                let linear_acceleration =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        linear_acceleration,
                    );
                let orientation =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        orientation,
                    );
                let angular_velocity =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        angular_velocity,
                    );
                let angular_acceleration =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        angular_acceleration,
                    );
                __widl_f_set_pose_VRMockDisplay(
                    self_,
                    position,
                    linear_velocity,
                    linear_acceleration,
                    orientation,
                    angular_velocity,
                    angular_acceleration,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_VRMockDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrMockDisplay as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockDisplay {
    #[cfg(all(feature = "VrMockDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/update)\n\n*This API requires the following crate features to be activated: `VrMockDisplay`*"]
    #[allow(clippy::all)]
    pub fn update(&self) {
        #[cfg(all(feature = "VrMockDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_VRMockDisplay(
                self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_VRMockDisplay(
            self_: <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrMockDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_update_VRMockDisplay(self_)
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
pub static __WASM_BINDGEN_GENERATED_f70175d8e7aa4a87: [u8; 776usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC6\x02\0\0\0\0\x06\0\0\x02\rVRMockDisplay\x1F__widl_instanceof_VRMockDisplay\0\0\0\0(__widl_f_set_eye_parameter_VRMockDisplay\0\0\0\x01\rVRMockDisplay\x01\0\0\x01\t\x05self_\x03eye\x08offset_x\x08offset_y\x08offset_z\tup_degree\x0Cright_degree\x0Bdown_degree\x0Bleft_degree\x0FsetEyeParameter\0\0\0)__widl_f_set_eye_resolution_VRMockDisplay\0\0\0\x01\rVRMockDisplay\x01\0\0\x01\x03\x05self_\x0Ea_render_width\x0Fa_render_height\x10setEyeResolution\0\0\0&__widl_f_set_mount_state_VRMockDisplay\0\0\0\x01\rVRMockDisplay\x01\0\0\x01\x02\x05self_\nis_mounted\rsetMountState\0\0\0\x1F__widl_f_set_pose_VRMockDisplay\0\0\0\x01\rVRMockDisplay\x01\0\0\x01\x07\x05self_\x08position\x0Flinear_velocity\x13linear_acceleration\x0Borientation\x10angular_velocity\x14angular_acceleration\x07setPose\0\0\0\x1D__widl_f_update_VRMockDisplay\0\0\0\x01\rVRMockDisplay\x01\0\0\x01\x01\x05self_\x06update\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
