use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Coordinates` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Coordinates {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Coordinates: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Coordinates {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(67u32);
            inform(111u32);
            inform(111u32);
            inform(114u32);
            inform(100u32);
            inform(105u32);
            inform(110u32);
            inform(97u32);
            inform(116u32);
            inform(101u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for Coordinates {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Coordinates {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Coordinates {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Coordinates {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Coordinates {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Coordinates {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Coordinates {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Coordinates {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Coordinates {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Coordinates>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Coordinates {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Coordinates {
        #[inline]
        fn from(obj: JsValue) -> Coordinates {
            Coordinates { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Coordinates {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Coordinates> for Coordinates {
        #[inline]
        fn as_ref(&self) -> &Coordinates {
            self
        }
    }
    impl From<Coordinates> for JsValue {
        #[inline]
        fn from(obj: Coordinates) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Coordinates {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Coordinates(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Coordinates(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Coordinates(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Coordinates { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Coordinates) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Coordinates> for ::js_sys::Object {
    #[inline]
    fn from(obj: Coordinates) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Coordinates {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_latitude_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `latitude` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/latitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn latitude(&self) -> f64 {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_latitude_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_latitude_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_latitude_Coordinates(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_longitude_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `longitude` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/longitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn longitude(&self) -> f64 {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_longitude_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_longitude_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_longitude_Coordinates(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_altitude_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `altitude` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitude)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn altitude(&self) -> Option<f64> {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_altitude_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_altitude_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_altitude_Coordinates(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_accuracy_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `accuracy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/accuracy)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn accuracy(&self) -> f64 {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_accuracy_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_accuracy_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_accuracy_Coordinates(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_altitude_accuracy_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `altitudeAccuracy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitudeAccuracy)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn altitude_accuracy(&self) -> Option<f64> {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_altitude_accuracy_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_altitude_accuracy_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_altitude_accuracy_Coordinates(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_heading_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `heading` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/heading)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn heading(&self) -> Option<f64> {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_heading_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_heading_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_heading_Coordinates(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Coordinates",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_speed_Coordinates() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Coordinates as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Coordinates {
    #[cfg(all(feature = "Coordinates",))]
    #[allow(bad_style)]
    #[doc = "The `speed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/speed)\n\n*This API requires the following crate features to be activated: `Coordinates`*"]
    #[allow(clippy::all)]
    pub fn speed(&self) -> Option<f64> {
        #[cfg(all(feature = "Coordinates",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_speed_Coordinates(
                self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_speed_Coordinates(
            self_: <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Coordinates as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_speed_Coordinates(self_)
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
pub static __WASM_BINDGEN_GENERATED_5ded9bd97aa7840a: [u8; 713usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x87\x02\0\0\0\0\x08\0\0\x02\x0BCoordinates\x1D__widl_instanceof_Coordinates\0\0\0\0\x1D__widl_f_latitude_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x08latitude\x01\x01\x05self_\x08latitude\0\0\0\x1E__widl_f_longitude_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\tlongitude\x01\x01\x05self_\tlongitude\0\0\0\x1D__widl_f_altitude_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x08altitude\x01\x01\x05self_\x08altitude\0\0\0\x1D__widl_f_accuracy_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x08accuracy\x01\x01\x05self_\x08accuracy\0\0\0&__widl_f_altitude_accuracy_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x10altitudeAccuracy\x01\x01\x05self_\x10altitudeAccuracy\0\0\0\x1C__widl_f_heading_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x07heading\x01\x01\x05self_\x07heading\0\0\0\x1A__widl_f_speed_Coordinates\0\0\0\x01\x0BCoordinates\x01\0\x01\x05speed\x01\x01\x05self_\x05speed\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
