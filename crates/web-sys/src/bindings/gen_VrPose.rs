use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRPose` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrPose {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrPose: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrPose {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(86u32);
            inform(82u32);
            inform(80u32);
            inform(111u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for VrPose {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrPose {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrPose {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrPose {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrPose {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrPose {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrPose {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrPose {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrPose {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrPose>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrPose {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrPose {
        #[inline]
        fn from(obj: JsValue) -> VrPose {
            VrPose { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrPose {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrPose> for VrPose {
        #[inline]
        fn as_ref(&self) -> &VrPose {
            self
        }
    }
    impl From<VrPose> for JsValue {
        #[inline]
        fn from(obj: VrPose) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrPose {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRPose(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRPose(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRPose(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrPose { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrPose) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrPose> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrPose) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrPose {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `position` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/position)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn position(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_linear_velocity_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `linearVelocity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearVelocity)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn linear_velocity(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_linear_velocity_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_linear_velocity_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_linear_velocity_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_linear_acceleration_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `linearAcceleration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearAcceleration)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn linear_acceleration(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_linear_acceleration_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_linear_acceleration_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_linear_acceleration_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `orientation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/orientation)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn orientation(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angular_velocity_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `angularVelocity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularVelocity)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn angular_velocity(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angular_velocity_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angular_velocity_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angular_velocity_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angular_acceleration_VRPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl VrPose {
    #[cfg(all(feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `angularAcceleration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularAcceleration)\n\n*This API requires the following crate features to be activated: `VrPose`*"]
    #[allow(clippy::all)]
    pub fn angular_acceleration(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angular_acceleration_VRPose(
                self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angular_acceleration_VRPose(
            self_: <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angular_acceleration_VRPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_70e08c136c15fd76: [u8; 664usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}V\x02\0\0\0\0\x07\0\0\x02\x06VRPose\x18__widl_instanceof_VRPose\0\0\0\0\x18__widl_f_position_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x08position\x01\x01\x05self_\x08position\0\0\0\x1F__widl_f_linear_velocity_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x0ElinearVelocity\x01\x01\x05self_\x0ElinearVelocity\0\0\0#__widl_f_linear_acceleration_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x12linearAcceleration\x01\x01\x05self_\x12linearAcceleration\0\0\0\x1B__widl_f_orientation_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x0Borientation\x01\x01\x05self_\x0Borientation\0\0\0 __widl_f_angular_velocity_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x0FangularVelocity\x01\x01\x05self_\x0FangularVelocity\0\0\0$__widl_f_angular_acceleration_VRPose\x01\0\0\x01\x06VRPose\x01\0\x01\x13angularAcceleration\x01\x01\x05self_\x13angularAcceleration\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
