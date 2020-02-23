use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadPose` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadPose {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadPose: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadPose {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(80u32);
            inform(111u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for GamepadPose {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadPose {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadPose {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadPose {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadPose {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadPose {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadPose {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadPose {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadPose {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadPose>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadPose {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadPose {
        #[inline]
        fn from(obj: JsValue) -> GamepadPose {
            GamepadPose { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadPose {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadPose> for GamepadPose {
        #[inline]
        fn as_ref(&self) -> &GamepadPose {
            self
        }
    }
    impl From<GamepadPose> for JsValue {
        #[inline]
        fn from(obj: GamepadPose) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadPose {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadPose(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadPose(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadPose(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadPose { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadPose) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadPose> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadPose) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadPose {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_orientation_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `hasOrientation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasOrientation)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn has_orientation(&self) -> bool {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_orientation_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_orientation_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_orientation_GamepadPose(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_position_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `hasPosition` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasPosition)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn has_position(&self) -> bool {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_position_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_position_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_position_GamepadPose(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `position` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/position)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn position(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_GamepadPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_linear_velocity_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `linearVelocity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearVelocity)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn linear_velocity(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_linear_velocity_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_linear_velocity_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_linear_velocity_GamepadPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_linear_acceleration_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `linearAcceleration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearAcceleration)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn linear_acceleration(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_linear_acceleration_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_linear_acceleration_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_linear_acceleration_GamepadPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `orientation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/orientation)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn orientation(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_GamepadPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angular_velocity_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `angularVelocity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularVelocity)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn angular_velocity(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angular_velocity_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angular_velocity_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angular_velocity_GamepadPose(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angular_acceleration_GamepadPose() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadPose as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl GamepadPose {
    #[cfg(all(feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `angularAcceleration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularAcceleration)\n\n*This API requires the following crate features to be activated: `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn angular_acceleration(&self) -> Result<Option<Vec<f32>>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angular_acceleration_GamepadPose(
                self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angular_acceleration_GamepadPose(
            self_: <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadPose as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angular_acceleration_GamepadPose(self_)
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
pub static __WASM_BINDGEN_GENERATED_f7879c90a47d362f: [u8; 919usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}U\x03\0\0\0\0\t\0\0\x02\x0BGamepadPose\x1D__widl_instanceof_GamepadPose\0\0\0\0$__widl_f_has_orientation_GamepadPose\0\0\0\x01\x0BGamepadPose\x01\0\x01\x0EhasOrientation\x01\x01\x05self_\x0EhasOrientation\0\0\0!__widl_f_has_position_GamepadPose\0\0\0\x01\x0BGamepadPose\x01\0\x01\x0BhasPosition\x01\x01\x05self_\x0BhasPosition\0\0\0\x1D__widl_f_position_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x08position\x01\x01\x05self_\x08position\0\0\0$__widl_f_linear_velocity_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x0ElinearVelocity\x01\x01\x05self_\x0ElinearVelocity\0\0\0(__widl_f_linear_acceleration_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x12linearAcceleration\x01\x01\x05self_\x12linearAcceleration\0\0\0 __widl_f_orientation_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x0Borientation\x01\x01\x05self_\x0Borientation\0\0\0%__widl_f_angular_velocity_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x0FangularVelocity\x01\x01\x05self_\x0FangularVelocity\0\0\0)__widl_f_angular_acceleration_GamepadPose\x01\0\0\x01\x0BGamepadPose\x01\0\x01\x13angularAcceleration\x01\x01\x05self_\x13angularAcceleration\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
