use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAngle` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAngle {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAngle: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAngle {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(65u32);
            inform(110u32);
            inform(103u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for SvgAngle {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAngle {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAngle {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAngle {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAngle {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAngle {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAngle {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAngle {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAngle {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAngle>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAngle {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAngle {
        #[inline]
        fn from(obj: JsValue) -> SvgAngle {
            SvgAngle { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAngle {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAngle> for SvgAngle {
        #[inline]
        fn as_ref(&self) -> &SvgAngle {
            self
        }
    }
    impl From<SvgAngle> for JsValue {
        #[inline]
        fn from(obj: SvgAngle) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAngle {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAngle(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAngle(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAngle(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAngle { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAngle) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAngle> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAngle) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAngle {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_to_specified_units_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAngle as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `convertToSpecifiedUnits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/convertToSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn convert_to_specified_units(
        &self,
        unit_type: u16,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_to_specified_units_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_to_specified_units_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(unit_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let unit_type = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unit_type);
                __widl_f_convert_to_specified_units_SVGAngle(self_, unit_type)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_value_specified_units_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgAngle as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `newValueSpecifiedUnits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/newValueSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn new_value_specified_units(
        &self,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_value_specified_units_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_value_specified_units_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(unit_type);
            drop(value_in_specified_units);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let unit_type = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unit_type);
                let value_in_specified_units =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_in_specified_units);
                __widl_f_new_value_specified_units_SVGAngle(
                    self_,
                    unit_type,
                    value_in_specified_units,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unit_type_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAngle as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `unitType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/unitType)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn unit_type(&self) -> u16 {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unit_type_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unit_type_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unit_type_SVGAngle(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAngle as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f32 {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_SVGAngle(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAngle as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f32) {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_SVGAngle(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_in_specified_units_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAngle as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `valueInSpecifiedUnits` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn value_in_specified_units(&self) -> f32 {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_in_specified_units_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_in_specified_units_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_in_specified_units_SVGAngle(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_in_specified_units_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAngle as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `valueInSpecifiedUnits` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn set_value_in_specified_units(&self, value_in_specified_units: f32) {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_in_specified_units_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_in_specified_units_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value_in_specified_units);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value_in_specified_units =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_in_specified_units);
                __widl_f_set_value_in_specified_units_SVGAngle(self_, value_in_specified_units)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_as_string_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAngle as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsString` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn value_as_string(&self) -> String {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_as_string_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_as_string_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_as_string_SVGAngle(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAngle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_as_string_SVGAngle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAngle as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAngle {
    #[cfg(all(feature = "SvgAngle",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsString` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)\n\n*This API requires the following crate features to be activated: `SvgAngle`*"]
    #[allow(clippy::all)]
    pub fn set_value_as_string(&self, value_as_string: &str) {
        #[cfg(all(feature = "SvgAngle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_as_string_SVGAngle(
                self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_as_string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_as_string_SVGAngle(
            self_: <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value_as_string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value_as_string);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgAngle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value_as_string =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_as_string);
                __widl_f_set_value_as_string_SVGAngle(self_, value_as_string)
            };
            ()
        }
    }
}
impl SvgAngle {
    pub const SVG_ANGLETYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgAngle {
    pub const SVG_ANGLETYPE_UNSPECIFIED: u16 = 1u64 as u16;
}
impl SvgAngle {
    pub const SVG_ANGLETYPE_DEG: u16 = 2u64 as u16;
}
impl SvgAngle {
    pub const SVG_ANGLETYPE_RAD: u16 = 3u64 as u16;
}
impl SvgAngle {
    pub const SVG_ANGLETYPE_GRAD: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_29ff800178f8c9a3: [u8; 1044usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD2\x03\0\0\0\0\n\0\0\x02\x08SVGAngle\x1A__widl_instanceof_SVGAngle\0\0\0\0,__widl_f_convert_to_specified_units_SVGAngle\x01\0\0\x01\x08SVGAngle\x01\0\0\x01\x02\x05self_\tunit_type\x17convertToSpecifiedUnits\0\0\0+__widl_f_new_value_specified_units_SVGAngle\x01\0\0\x01\x08SVGAngle\x01\0\0\x01\x03\x05self_\tunit_type\x18value_in_specified_units\x16newValueSpecifiedUnits\0\0\0\x1B__widl_f_unit_type_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x01\x08unitType\x01\x01\x05self_\x08unitType\0\0\0\x17__widl_f_value_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\x1B__widl_f_set_value_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0*__widl_f_value_in_specified_units_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x01\x15valueInSpecifiedUnits\x01\x01\x05self_\x15valueInSpecifiedUnits\0\0\0.__widl_f_set_value_in_specified_units_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x02\x15valueInSpecifiedUnits\x01\x02\x05self_\x18value_in_specified_units\x15valueInSpecifiedUnits\0\0\0!__widl_f_value_as_string_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x01\rvalueAsString\x01\x01\x05self_\rvalueAsString\0\0\0%__widl_f_set_value_as_string_SVGAngle\0\0\0\x01\x08SVGAngle\x01\0\x02\rvalueAsString\x01\x02\x05self_\x0Fvalue_as_string\rvalueAsString\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
