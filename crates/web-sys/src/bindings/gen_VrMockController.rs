use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRMockController` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrMockController {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrMockController: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrMockController {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(86u32);
            inform(82u32);
            inform(77u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for VrMockController {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrMockController {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrMockController {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrMockController {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrMockController {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrMockController {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrMockController {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrMockController {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrMockController {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrMockController>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrMockController {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrMockController {
        #[inline]
        fn from(obj: JsValue) -> VrMockController {
            VrMockController { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrMockController {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrMockController> for VrMockController {
        #[inline]
        fn as_ref(&self) -> &VrMockController {
            self
        }
    }
    impl From<VrMockController> for JsValue {
        #[inline]
        fn from(obj: VrMockController) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrMockController {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRMockController(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRMockController(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRMockController(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrMockController { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrMockController) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrMockController> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrMockController) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrMockController {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrMockController",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_axis_move_event_VRMockController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&VrMockController as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockController {
    #[cfg(all(feature = "VrMockController",))]
    #[allow(bad_style)]
    #[doc = "The `newAxisMoveEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    #[allow(clippy::all)]
    pub fn new_axis_move_event(&self, axis: u32, value: f64) {
        #[cfg(all(feature = "VrMockController",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_axis_move_event_VRMockController(
                self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                axis: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_axis_move_event_VRMockController(
            self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            axis: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(axis);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let axis = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(axis);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_new_axis_move_event_VRMockController(self_, axis, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockController",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_button_event_VRMockController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&VrMockController as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockController {
    #[cfg(all(feature = "VrMockController",))]
    #[allow(bad_style)]
    #[doc = "The `newButtonEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newButtonEvent)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    #[allow(clippy::all)]
    pub fn new_button_event(&self, button: u32, pressed: bool) {
        #[cfg(all(feature = "VrMockController",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_button_event_VRMockController(
                self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_button_event_VRMockController(
            self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(button);
            drop(pressed);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let button = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                let pressed = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pressed);
                __widl_f_new_button_event_VRMockController(self_, button, pressed)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrMockController",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_pose_move_VRMockController() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&VrMockController as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrMockController {
    #[cfg(all(feature = "VrMockController",))]
    #[allow(bad_style)]
    #[doc = "The `newPoseMove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    #[allow(clippy::all)]
    pub fn new_pose_move(
        &self,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    ) {
        #[cfg(all(feature = "VrMockController",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_pose_move_VRMockController(
                self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                position: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                linear_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                linear_acceleration : < Option < & mut [ f32 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                orientation: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angular_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                angular_acceleration : < Option < & mut [ f32 ] > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_pose_move_VRMockController(
            self_: <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&VrMockController as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
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
                __widl_f_new_pose_move_VRMockController(
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
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4cd58f74e70236a6: [u8; 560usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEE\x01\0\0\0\0\x04\0\0\x02\x10VRMockController\"__widl_instanceof_VRMockController\0\0\0\0-__widl_f_new_axis_move_event_VRMockController\0\0\0\x01\x10VRMockController\x01\0\0\x01\x03\x05self_\x04axis\x05value\x10newAxisMoveEvent\0\0\0*__widl_f_new_button_event_VRMockController\0\0\0\x01\x10VRMockController\x01\0\0\x01\x03\x05self_\x06button\x07pressed\x0EnewButtonEvent\0\0\0'__widl_f_new_pose_move_VRMockController\0\0\0\x01\x10VRMockController\x01\0\0\x01\x07\x05self_\x08position\x0Flinear_velocity\x13linear_acceleration\x0Borientation\x10angular_velocity\x14angular_acceleration\x0BnewPoseMove\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
