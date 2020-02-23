use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceRotationRate` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceRotationRate {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceRotationRate: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceRotationRate {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(82u32);
            inform(111u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(82u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for DeviceRotationRate {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceRotationRate {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceRotationRate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceRotationRate {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceRotationRate {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceRotationRate {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceRotationRate {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceRotationRate {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceRotationRate {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceRotationRate>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceRotationRate {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceRotationRate {
        #[inline]
        fn from(obj: JsValue) -> DeviceRotationRate {
            DeviceRotationRate { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceRotationRate {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceRotationRate> for DeviceRotationRate {
        #[inline]
        fn as_ref(&self) -> &DeviceRotationRate {
            self
        }
    }
    impl From<DeviceRotationRate> for JsValue {
        #[inline]
        fn from(obj: DeviceRotationRate) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceRotationRate {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceRotationRate(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceRotationRate(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceRotationRate(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceRotationRate { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceRotationRate) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceRotationRate> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceRotationRate) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceRotationRate {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceRotationRate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alpha_DeviceRotationRate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceRotationRate as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceRotationRate {
    #[cfg(all(feature = "DeviceRotationRate",))]
    #[allow(bad_style)]
    #[doc = "The `alpha` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/alpha)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    #[allow(clippy::all)]
    pub fn alpha(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceRotationRate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alpha_DeviceRotationRate(
                self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alpha_DeviceRotationRate(
            self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alpha_DeviceRotationRate(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceRotationRate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_beta_DeviceRotationRate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceRotationRate as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceRotationRate {
    #[cfg(all(feature = "DeviceRotationRate",))]
    #[allow(bad_style)]
    #[doc = "The `beta` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/beta)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    #[allow(clippy::all)]
    pub fn beta(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceRotationRate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_beta_DeviceRotationRate(
                self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_beta_DeviceRotationRate(
            self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_beta_DeviceRotationRate(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceRotationRate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_gamma_DeviceRotationRate() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceRotationRate as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceRotationRate {
    #[cfg(all(feature = "DeviceRotationRate",))]
    #[allow(bad_style)]
    #[doc = "The `gamma` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/gamma)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    #[allow(clippy::all)]
    pub fn gamma(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceRotationRate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_gamma_DeviceRotationRate(
                self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_gamma_DeviceRotationRate(
            self_: <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceRotationRate as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_gamma_DeviceRotationRate(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1fc76000f23dab1c: [u8; 411usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Y\x01\0\0\0\0\x04\0\0\x02\x12DeviceRotationRate$__widl_instanceof_DeviceRotationRate\0\0\0\0!__widl_f_alpha_DeviceRotationRate\0\0\0\x01\x12DeviceRotationRate\x01\0\x01\x05alpha\x01\x01\x05self_\x05alpha\0\0\0 __widl_f_beta_DeviceRotationRate\0\0\0\x01\x12DeviceRotationRate\x01\0\x01\x04beta\x01\x01\x05self_\x04beta\0\0\0!__widl_f_gamma_DeviceRotationRate\0\0\0\x01\x12DeviceRotationRate\x01\0\x01\x05gamma\x01\x01\x05self_\x05gamma\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
