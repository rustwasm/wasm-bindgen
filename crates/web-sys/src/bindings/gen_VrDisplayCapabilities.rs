use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VRDisplayCapabilities` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VrDisplayCapabilities {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VrDisplayCapabilities: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VrDisplayCapabilities {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(86u32);
            inform(82u32);
            inform(68u32);
            inform(105u32);
            inform(115u32);
            inform(112u32);
            inform(108u32);
            inform(97u32);
            inform(121u32);
            inform(67u32);
            inform(97u32);
            inform(112u32);
            inform(97u32);
            inform(98u32);
            inform(105u32);
            inform(108u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(101u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for VrDisplayCapabilities {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VrDisplayCapabilities {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VrDisplayCapabilities {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VrDisplayCapabilities {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VrDisplayCapabilities {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VrDisplayCapabilities {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VrDisplayCapabilities {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VrDisplayCapabilities {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VrDisplayCapabilities {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VrDisplayCapabilities>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VrDisplayCapabilities {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VrDisplayCapabilities {
        #[inline]
        fn from(obj: JsValue) -> VrDisplayCapabilities {
            VrDisplayCapabilities { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VrDisplayCapabilities {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VrDisplayCapabilities> for VrDisplayCapabilities {
        #[inline]
        fn as_ref(&self) -> &VrDisplayCapabilities {
            self
        }
    }
    impl From<VrDisplayCapabilities> for JsValue {
        #[inline]
        fn from(obj: VrDisplayCapabilities) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VrDisplayCapabilities {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VRDisplayCapabilities(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VRDisplayCapabilities(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VRDisplayCapabilities(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VrDisplayCapabilities { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VrDisplayCapabilities) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VrDisplayCapabilities> for ::js_sys::Object {
    #[inline]
    fn from(obj: VrDisplayCapabilities) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VrDisplayCapabilities {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_position_VRDisplayCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplayCapabilities as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplayCapabilities {
    #[cfg(all(feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `hasPosition` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasPosition)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn has_position(&self) -> bool {
        #[cfg(all(feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_position_VRDisplayCapabilities(
                self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_position_VRDisplayCapabilities(
            self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_position_VRDisplayCapabilities(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_orientation_VRDisplayCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplayCapabilities as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplayCapabilities {
    #[cfg(all(feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `hasOrientation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasOrientation)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn has_orientation(&self) -> bool {
        #[cfg(all(feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_orientation_VRDisplayCapabilities(
                self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_orientation_VRDisplayCapabilities(
            self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_orientation_VRDisplayCapabilities(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_external_display_VRDisplayCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplayCapabilities as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplayCapabilities {
    #[cfg(all(feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `hasExternalDisplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasExternalDisplay)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn has_external_display(&self) -> bool {
        #[cfg(all(feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_external_display_VRDisplayCapabilities(
                self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_external_display_VRDisplayCapabilities(
            self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_has_external_display_VRDisplayCapabilities(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_can_present_VRDisplayCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplayCapabilities as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VrDisplayCapabilities {
    #[cfg(all(feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `canPresent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/canPresent)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn can_present(&self) -> bool {
        #[cfg(all(feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_can_present_VRDisplayCapabilities(
                self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_can_present_VRDisplayCapabilities(
            self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_can_present_VRDisplayCapabilities(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VrDisplayCapabilities",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_layers_VRDisplayCapabilities() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VrDisplayCapabilities as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VrDisplayCapabilities {
    #[cfg(all(feature = "VrDisplayCapabilities",))]
    #[allow(bad_style)]
    #[doc = "The `maxLayers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/maxLayers)\n\n*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    #[allow(clippy::all)]
    pub fn max_layers(&self) -> u32 {
        #[cfg(all(feature = "VrDisplayCapabilities",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_layers_VRDisplayCapabilities(
                self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_layers_VRDisplayCapabilities(
            self_: <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VrDisplayCapabilities as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_layers_VRDisplayCapabilities(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f04aeafca7d7a1c1: [u8; 733usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9B\x02\0\0\0\0\x06\0\0\x02\x15VRDisplayCapabilities'__widl_instanceof_VRDisplayCapabilities\0\0\0\0+__widl_f_has_position_VRDisplayCapabilities\0\0\0\x01\x15VRDisplayCapabilities\x01\0\x01\x0BhasPosition\x01\x01\x05self_\x0BhasPosition\0\0\0.__widl_f_has_orientation_VRDisplayCapabilities\0\0\0\x01\x15VRDisplayCapabilities\x01\0\x01\x0EhasOrientation\x01\x01\x05self_\x0EhasOrientation\0\0\03__widl_f_has_external_display_VRDisplayCapabilities\0\0\0\x01\x15VRDisplayCapabilities\x01\0\x01\x12hasExternalDisplay\x01\x01\x05self_\x12hasExternalDisplay\0\0\0*__widl_f_can_present_VRDisplayCapabilities\0\0\0\x01\x15VRDisplayCapabilities\x01\0\x01\ncanPresent\x01\x01\x05self_\ncanPresent\0\0\0)__widl_f_max_layers_VRDisplayCapabilities\0\0\0\x01\x15VRDisplayCapabilities\x01\0\x01\tmaxLayers\x01\x01\x05self_\tmaxLayers\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
