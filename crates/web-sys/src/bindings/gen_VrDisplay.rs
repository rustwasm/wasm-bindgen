use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRDisplay` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrDisplay {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrDisplay: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrDisplay {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(86u32);
            inform(82u32);
            inform(68u32);
            inform(105u32);
            inform(115u32);
            inform(112u32);
            inform(108u32);
            inform(97u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for VrDisplay {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrDisplay {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrDisplay {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrDisplay {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrDisplay {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrDisplay {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrDisplay {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrDisplay {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrDisplay {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrDisplay>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrDisplay {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrDisplay {
        #[inline]
        fn from(obj: JsValue) -> VrDisplay {
            VrDisplay { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrDisplay {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrDisplay> for VrDisplay {
        #[inline]
        fn as_ref(&self) -> &VrDisplay {
            self
        }
    }
    impl From<VrDisplay> for JsValue {
        #[inline]
        fn from(obj: VrDisplay) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrDisplay {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRDisplay(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRDisplay(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRDisplay(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrDisplay { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrDisplay) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrDisplay> for EventTarget {
    #[inline]
    fn from(obj: VrDisplay) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for VrDisplay {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<VrDisplay> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrDisplay) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrDisplay {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_animation_frame_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `cancelAnimationFrame()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/cancelAnimationFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn cancel_animation_frame(&self, handle: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancel_animation_frame_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancel_animation_frame_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            handle: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(handle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let handle = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(handle);
                __widl_f_cancel_animation_frame_VRDisplay(self_, handle)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exit_present_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `exitPresent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/exitPresent)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn exit_present(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exit_present_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exit_present_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_exit_present_VRDisplay(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrEye", feature = "VrEyeParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_eye_parameters_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <VrEye as WasmDescribe>::describe();
    <VrEyeParameters as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrEye", feature = "VrEyeParameters",))]
    #[allow(bad_style)]
    #[doc = "The `getEyeParameters()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getEyeParameters)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrEye`, `VrEyeParameters`*"]
    #[allow(clippy::all)]
    pub fn get_eye_parameters(&self, which_eye: VrEye) -> VrEyeParameters {
        #[cfg(all(feature = "VrDisplay", feature = "VrEye", feature = "VrEyeParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_eye_parameters_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                which_eye: <VrEye as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrEyeParameters as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_eye_parameters_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            which_eye: <VrEye as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrEyeParameters as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(which_eye);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let which_eye = <VrEye as wasm_bindgen::convert::IntoWasmAbi>::into_abi(which_eye);
                __widl_f_get_eye_parameters_VRDisplay(self_, which_eye)
            };
            <VrEyeParameters as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrFrameData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_frame_data_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <&VrFrameData as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrFrameData",))]
    #[allow(bad_style)]
    #[doc = "The `getFrameData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getFrameData)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrFrameData`*"]
    #[allow(clippy::all)]
    pub fn get_frame_data(&self, frame_data: &VrFrameData) -> bool {
        #[cfg(all(feature = "VrDisplay", feature = "VrFrameData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_frame_data_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frame_data: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_frame_data_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frame_data: <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(frame_data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frame_data =
                    <&VrFrameData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frame_data);
                __widl_f_get_frame_data_VRDisplay(self_, frame_data)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_layers_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `getLayers()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getLayers)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn get_layers(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_layers_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_layers_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_layers_VRDisplay(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_pose_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <VrPose as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrPose",))]
    #[allow(bad_style)]
    #[doc = "The `getPose()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getPose)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrPose`*"]
    #[allow(clippy::all)]
    pub fn get_pose(&self) -> VrPose {
        #[cfg(all(feature = "VrDisplay", feature = "VrPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_pose_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrPose as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_pose_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrPose as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_pose_VRDisplay(self_)
            };
            <VrPose as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrSubmitFrameResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_submit_frame_result_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <&VrSubmitFrameResult as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrSubmitFrameResult",))]
    #[allow(bad_style)]
    #[doc = "The `getSubmitFrameResult()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrSubmitFrameResult`*"]
    #[allow(clippy::all)]
    pub fn get_submit_frame_result(&self, result: &VrSubmitFrameResult) -> bool {
        #[cfg(all(feature = "VrDisplay", feature = "VrSubmitFrameResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_submit_frame_result_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                result: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_submit_frame_result_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            result: <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(result);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let result =
                    <&VrSubmitFrameResult as wasm_bindgen::convert::IntoWasmAbi>::into_abi(result);
                __widl_f_get_submit_frame_result_VRDisplay(self_, result)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_animation_frame_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `requestAnimationFrame()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestAnimationFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn request_animation_frame(
        &self,
        callback: &::js_sys::Function,
    ) -> Result<i32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_animation_frame_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_animation_frame_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                __widl_f_request_animation_frame_VRDisplay(self_, callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_present_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `requestPresent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestPresent)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn request_present(
        &self,
        layers: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_present_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                layers: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_present_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            layers: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(layers);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let layers =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        layers,
                    );
                __widl_f_request_present_VRDisplay(self_, layers)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reset_pose_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `resetPose()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/resetPose)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn reset_pose(&self) {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reset_pose_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reset_pose_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reset_pose_VRDisplay(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_submit_frame_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `submitFrame()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/submitFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn submit_frame(&self) {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_submit_frame_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_submit_frame_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_submit_frame_VRDisplay(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_connected_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `isConnected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isConnected)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn is_connected(&self) -> bool {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_connected_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_connected_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_connected_VRDisplay(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_presenting_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `isPresenting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isPresenting)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn is_presenting(&self) -> bool {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_presenting_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_presenting_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_presenting_VRDisplay(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_capabilities_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <VrDisplayCapabilities as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `capabilities` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/capabilities)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn capabilities(&self) -> VrDisplayCapabilities {
        #[cfg(all(feature = "VrDisplay", feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_capabilities_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrDisplayCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_capabilities_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrDisplayCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_capabilities_VRDisplay(self_)
            };
            <VrDisplayCapabilities as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay", feature = "VrStageParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stage_parameters_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <Option<VrStageParameters> as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay", feature = "VrStageParameters",))]
    #[allow(bad_style)]
    #[doc = "The `stageParameters` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/stageParameters)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrStageParameters`*"]
    #[allow(clippy::all)]
    pub fn stage_parameters(&self) -> Option<VrStageParameters> {
        #[cfg(all(feature = "VrDisplay", feature = "VrStageParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stage_parameters_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<VrStageParameters> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stage_parameters_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<VrStageParameters> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stage_parameters_VRDisplay(self_)
            };
            <Option<VrStageParameters> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_display_id_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `displayId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayId)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn display_id(&self) -> u32 {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_display_id_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_display_id_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_display_id_VRDisplay(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_display_name_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `displayName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayName)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn display_name(&self) -> String {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_display_name_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_display_name_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_display_name_VRDisplay(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_depth_near_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `depthNear` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn depth_near(&self) -> f64 {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_depth_near_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_depth_near_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_depth_near_VRDisplay(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_depth_near_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `depthNear` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_depth_near(&self, depth_near: f64) {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_depth_near_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                depth_near: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_depth_near_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            depth_near: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(depth_near);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let depth_near = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(depth_near);
                __widl_f_set_depth_near_VRDisplay(self_, depth_near)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_depth_far_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplay as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `depthFar` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn depth_far(&self) -> f64 {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_depth_far_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_depth_far_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_depth_far_VRDisplay(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplay",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_depth_far_VRDisplay() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VrDisplay as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VrDisplay {
    #[cfg(all(feature = "VrDisplay",))]
    #[allow(bad_style)]
    #[doc = "The `depthFar` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    #[allow(clippy::all)]
    pub fn set_depth_far(&self, depth_far: f64) {
        #[cfg(all(feature = "VrDisplay",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_depth_far_VRDisplay(
                self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                depth_far: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_depth_far_VRDisplay(
            self_: <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            depth_far: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(depth_far);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrDisplay as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let depth_far = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(depth_far);
                __widl_f_set_depth_far_VRDisplay(self_, depth_far)
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
pub static __WASM_BINDGEN_GENERATED_59c5044251125224: [u8; 1904usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}.\x07\0\0\0\0\x16\0\0\x02\tVRDisplay\x1B__widl_instanceof_VRDisplay\0\0\0\0)__widl_f_cancel_animation_frame_VRDisplay\x01\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\x06handle\x14cancelAnimationFrame\0\0\0\x1F__widl_f_exit_present_VRDisplay\x01\0\0\x01\tVRDisplay\x01\0\0\x01\x01\x05self_\x0BexitPresent\0\0\0%__widl_f_get_eye_parameters_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\twhich_eye\x10getEyeParameters\0\0\0!__widl_f_get_frame_data_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\nframe_data\x0CgetFrameData\0\0\0\x1D__widl_f_get_layers_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x01\x05self_\tgetLayers\0\0\0\x1B__widl_f_get_pose_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x01\x05self_\x07getPose\0\0\0*__widl_f_get_submit_frame_result_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\x06result\x14getSubmitFrameResult\0\0\0*__widl_f_request_animation_frame_VRDisplay\x01\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\x08callback\x15requestAnimationFrame\0\0\0\"__widl_f_request_present_VRDisplay\x01\0\0\x01\tVRDisplay\x01\0\0\x01\x02\x05self_\x06layers\x0ErequestPresent\0\0\0\x1D__widl_f_reset_pose_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x01\x05self_\tresetPose\0\0\0\x1F__widl_f_submit_frame_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\0\x01\x01\x05self_\x0BsubmitFrame\0\0\0\x1F__widl_f_is_connected_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x0BisConnected\x01\x01\x05self_\x0BisConnected\0\0\0 __widl_f_is_presenting_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x0CisPresenting\x01\x01\x05self_\x0CisPresenting\0\0\0\x1F__widl_f_capabilities_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x0Ccapabilities\x01\x01\x05self_\x0Ccapabilities\0\0\0#__widl_f_stage_parameters_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x0FstageParameters\x01\x01\x05self_\x0FstageParameters\0\0\0\x1D__widl_f_display_id_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\tdisplayId\x01\x01\x05self_\tdisplayId\0\0\0\x1F__widl_f_display_name_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x0BdisplayName\x01\x01\x05self_\x0BdisplayName\0\0\0\x1D__widl_f_depth_near_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\tdepthNear\x01\x01\x05self_\tdepthNear\0\0\0!__widl_f_set_depth_near_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x02\tdepthNear\x01\x02\x05self_\ndepth_near\tdepthNear\0\0\0\x1C__widl_f_depth_far_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x01\x08depthFar\x01\x01\x05self_\x08depthFar\0\0\0 __widl_f_set_depth_far_VRDisplay\0\0\0\x01\tVRDisplay\x01\0\x02\x08depthFar\x01\x02\x05self_\tdepth_far\x08depthFar\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
