use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceAcceleration` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceAcceleration {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceAcceleration: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceAcceleration {
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
            inform(65u32);
            inform(99u32);
            inform(99u32);
            inform(101u32);
            inform(108u32);
            inform(101u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for DeviceAcceleration {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceAcceleration {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceAcceleration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceAcceleration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceAcceleration {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceAcceleration {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceAcceleration {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceAcceleration {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceAcceleration {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceAcceleration>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceAcceleration {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceAcceleration {
        #[inline]
        fn from(obj: JsValue) -> DeviceAcceleration {
            DeviceAcceleration { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceAcceleration {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceAcceleration> for DeviceAcceleration {
        #[inline]
        fn as_ref(&self) -> &DeviceAcceleration {
            self
        }
    }
    impl From<DeviceAcceleration> for JsValue {
        #[inline]
        fn from(obj: DeviceAcceleration) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceAcceleration {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceAcceleration(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceAcceleration(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceAcceleration(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceAcceleration { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceAcceleration) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceAcceleration> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceAcceleration) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceAcceleration {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceAcceleration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_DeviceAcceleration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceAcceleration as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceAcceleration {
    #[cfg(all(feature = "DeviceAcceleration",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/x)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceAcceleration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_DeviceAcceleration(
                self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_DeviceAcceleration(
            self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_DeviceAcceleration(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceAcceleration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_DeviceAcceleration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceAcceleration as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceAcceleration {
    #[cfg(all(feature = "DeviceAcceleration",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/y)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceAcceleration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_DeviceAcceleration(
                self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_DeviceAcceleration(
            self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_DeviceAcceleration(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceAcceleration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_z_DeviceAcceleration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceAcceleration as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceAcceleration {
    #[cfg(all(feature = "DeviceAcceleration",))]
    #[allow(bad_style)]
    #[doc = "The `z` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/z)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    #[allow(clippy::all)]
    pub fn z(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceAcceleration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_z_DeviceAcceleration(
                self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_z_DeviceAcceleration(
            self_: <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceAcceleration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_z_DeviceAcceleration(self_)
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
pub static __WASM_BINDGEN_GENERATED_7fade0b4a69b1658: [u8; 378usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}8\x01\0\0\0\0\x04\0\0\x02\x12DeviceAcceleration$__widl_instanceof_DeviceAcceleration\0\0\0\0\x1D__widl_f_x_DeviceAcceleration\0\0\0\x01\x12DeviceAcceleration\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1D__widl_f_y_DeviceAcceleration\0\0\0\x01\x12DeviceAcceleration\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1D__widl_f_z_DeviceAcceleration\0\0\0\x01\x12DeviceAcceleration\x01\0\x01\x01z\x01\x01\x05self_\x01z\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
