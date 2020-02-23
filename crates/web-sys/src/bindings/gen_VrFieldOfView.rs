use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRFieldOfView` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrFieldOfView {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrFieldOfView: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrFieldOfView {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(86u32);
            inform(82u32);
            inform(70u32);
            inform(105u32);
            inform(101u32);
            inform(108u32);
            inform(100u32);
            inform(79u32);
            inform(102u32);
            inform(86u32);
            inform(105u32);
            inform(101u32);
            inform(119u32);
        }
    }
    impl core::ops::Deref for VrFieldOfView {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrFieldOfView {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrFieldOfView {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrFieldOfView {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrFieldOfView {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrFieldOfView {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrFieldOfView {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrFieldOfView {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrFieldOfView {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrFieldOfView>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrFieldOfView {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrFieldOfView {
        #[inline]
        fn from(obj: JsValue) -> VrFieldOfView {
            VrFieldOfView { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrFieldOfView {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrFieldOfView> for VrFieldOfView {
        #[inline]
        fn as_ref(&self) -> &VrFieldOfView {
            self
        }
    }
    impl From<VrFieldOfView> for JsValue {
        #[inline]
        fn from(obj: VrFieldOfView) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrFieldOfView {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRFieldOfView(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRFieldOfView(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRFieldOfView(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrFieldOfView { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrFieldOfView) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrFieldOfView> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrFieldOfView) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrFieldOfView {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrFieldOfView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_up_degrees_VRFieldOfView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFieldOfView as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrFieldOfView {
    #[cfg(all(feature = "VrFieldOfView",))]
    #[allow(bad_style)]
    #[doc = "The `upDegrees` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/upDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    #[allow(clippy::all)]
    pub fn up_degrees(&self) -> f64 {
        #[cfg(all(feature = "VrFieldOfView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_up_degrees_VRFieldOfView(
                self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_up_degrees_VRFieldOfView(
            self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_up_degrees_VRFieldOfView(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrFieldOfView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_right_degrees_VRFieldOfView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFieldOfView as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrFieldOfView {
    #[cfg(all(feature = "VrFieldOfView",))]
    #[allow(bad_style)]
    #[doc = "The `rightDegrees` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/rightDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    #[allow(clippy::all)]
    pub fn right_degrees(&self) -> f64 {
        #[cfg(all(feature = "VrFieldOfView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_right_degrees_VRFieldOfView(
                self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_right_degrees_VRFieldOfView(
            self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_right_degrees_VRFieldOfView(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrFieldOfView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_down_degrees_VRFieldOfView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFieldOfView as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrFieldOfView {
    #[cfg(all(feature = "VrFieldOfView",))]
    #[allow(bad_style)]
    #[doc = "The `downDegrees` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/downDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    #[allow(clippy::all)]
    pub fn down_degrees(&self) -> f64 {
        #[cfg(all(feature = "VrFieldOfView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_down_degrees_VRFieldOfView(
                self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_down_degrees_VRFieldOfView(
            self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_down_degrees_VRFieldOfView(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrFieldOfView",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_degrees_VRFieldOfView() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrFieldOfView as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VrFieldOfView {
    #[cfg(all(feature = "VrFieldOfView",))]
    #[allow(bad_style)]
    #[doc = "The `leftDegrees` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/leftDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    #[allow(clippy::all)]
    pub fn left_degrees(&self) -> f64 {
        #[cfg(all(feature = "VrFieldOfView",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_degrees_VRFieldOfView(
                self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_degrees_VRFieldOfView(
            self_: <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VrFieldOfView as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_degrees_VRFieldOfView(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a8cffe99eb0ff29b: [u8; 520usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC6\x01\0\0\0\0\x05\0\0\x02\rVRFieldOfView\x1F__widl_instanceof_VRFieldOfView\0\0\0\0!__widl_f_up_degrees_VRFieldOfView\0\0\0\x01\rVRFieldOfView\x01\0\x01\tupDegrees\x01\x01\x05self_\tupDegrees\0\0\0$__widl_f_right_degrees_VRFieldOfView\0\0\0\x01\rVRFieldOfView\x01\0\x01\x0CrightDegrees\x01\x01\x05self_\x0CrightDegrees\0\0\0#__widl_f_down_degrees_VRFieldOfView\0\0\0\x01\rVRFieldOfView\x01\0\x01\x0BdownDegrees\x01\x01\x05self_\x0BdownDegrees\0\0\0#__widl_f_left_degrees_VRFieldOfView\0\0\0\x01\rVRFieldOfView\x01\0\x01\x0BleftDegrees\x01\x01\x05self_\x0BleftDegrees\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
