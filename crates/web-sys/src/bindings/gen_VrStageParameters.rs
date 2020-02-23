use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRStageParameters` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters)\n\n*This API requires the following crate features to be activated: `VrStageParameters`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrStageParameters {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrStageParameters: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrStageParameters {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(86u32);
            inform(82u32);
            inform(83u32);
            inform(116u32);
            inform(97u32);
            inform(103u32);
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
    impl core::ops::Deref for VrStageParameters {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrStageParameters {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrStageParameters {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrStageParameters {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrStageParameters {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrStageParameters {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrStageParameters {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrStageParameters {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrStageParameters {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrStageParameters>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrStageParameters {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrStageParameters {
        #[inline]
        fn from(obj: JsValue) -> VrStageParameters {
            VrStageParameters { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrStageParameters {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrStageParameters> for VrStageParameters {
        #[inline]
        fn as_ref(&self) -> &VrStageParameters {
            self
        }
    }
    impl From<VrStageParameters> for JsValue {
        #[inline]
        fn from(obj: VrStageParameters) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrStageParameters {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRStageParameters(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRStageParameters(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRStageParameters(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrStageParameters { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrStageParameters) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrStageParameters> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrStageParameters) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrStageParameters {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrStageParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sitting_to_standing_transform_VRStageParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrStageParameters as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl VrStageParameters {
    #[cfg(all(feature = "VrStageParameters",))]
    #[allow(bad_style)]
    #[doc = "The `sittingToStandingTransform` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sittingToStandingTransform)\n\n*This API requires the following crate features to be activated: `VrStageParameters`*"]
    #[allow(clippy::all)]
    pub fn sitting_to_standing_transform(&self) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VrStageParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sitting_to_standing_transform_VRStageParameters(
                self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sitting_to_standing_transform_VRStageParameters(
            self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sitting_to_standing_transform_VRStageParameters(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VrStageParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_x_VRStageParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrStageParameters as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl VrStageParameters {
    #[cfg(all(feature = "VrStageParameters",))]
    #[allow(bad_style)]
    #[doc = "The `sizeX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeX)\n\n*This API requires the following crate features to be activated: `VrStageParameters`*"]
    #[allow(clippy::all)]
    pub fn size_x(&self) -> f32 {
        #[cfg(all(feature = "VrStageParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_x_VRStageParameters(
                self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_x_VRStageParameters(
            self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_x_VRStageParameters(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrStageParameters",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_z_VRStageParameters() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrStageParameters as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl VrStageParameters {
    #[cfg(all(feature = "VrStageParameters",))]
    #[allow(bad_style)]
    #[doc = "The `sizeZ` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeZ)\n\n*This API requires the following crate features to be activated: `VrStageParameters`*"]
    #[allow(clippy::all)]
    pub fn size_z(&self) -> f32 {
        #[cfg(all(feature = "VrStageParameters",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_z_VRStageParameters(
                self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_z_VRStageParameters(
            self_: <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrStageParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_z_VRStageParameters(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ad17368d7882055a: [u8; 474usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x98\x01\0\0\0\0\x04\0\0\x02\x11VRStageParameters#__widl_instanceof_VRStageParameters\0\0\0\08__widl_f_sitting_to_standing_transform_VRStageParameters\x01\0\0\x01\x11VRStageParameters\x01\0\x01\x1AsittingToStandingTransform\x01\x01\x05self_\x1AsittingToStandingTransform\0\0\0!__widl_f_size_x_VRStageParameters\0\0\0\x01\x11VRStageParameters\x01\0\x01\x05sizeX\x01\x01\x05self_\x05sizeX\0\0\0!__widl_f_size_z_VRStageParameters\0\0\0\x01\x11VRStageParameters\x01\0\x01\x05sizeZ\x01\x01\x05self_\x05sizeZ\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
