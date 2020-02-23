use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGLength` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgLength {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgLength: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgLength {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(76u32);
            inform(101u32);
            inform(110u32);
            inform(103u32);
            inform(116u32);
            inform(104u32);
        }
    }
    impl core::ops::Deref for SvgLength {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgLength {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgLength {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgLength {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgLength {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgLength {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgLength {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgLength {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgLength {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgLength>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgLength {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgLength {
        #[inline]
        fn from(obj: JsValue) -> SvgLength {
            SvgLength { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgLength {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgLength> for SvgLength {
        #[inline]
        fn as_ref(&self) -> &SvgLength {
            self
        }
    }
    impl From<SvgLength> for JsValue {
        #[inline]
        fn from(obj: SvgLength) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgLength {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGLength(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGLength(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGLength(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgLength { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgLength) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgLength> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgLength) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgLength {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_convert_to_specified_units_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgLength as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `convertToSpecifiedUnits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/convertToSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn convert_to_specified_units(
        &self,
        unit_type: u16,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_convert_to_specified_units_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_convert_to_specified_units_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let unit_type = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unit_type);
                __widl_f_convert_to_specified_units_SVGLength(self_, unit_type)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_value_specified_units_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SvgLength as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `newValueSpecifiedUnits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/newValueSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn new_value_specified_units(
        &self,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_value_specified_units_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unit_type: <u16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_value_specified_units_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let unit_type = <u16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unit_type);
                let value_in_specified_units =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_in_specified_units);
                __widl_f_new_value_specified_units_SVGLength(
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
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unit_type_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLength as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `unitType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/unitType)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn unit_type(&self) -> u16 {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unit_type_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unit_type_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unit_type_SVGLength(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLength as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_SVGLength(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgLength as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_SVGLength(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_in_specified_units_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLength as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `valueInSpecifiedUnits` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn value_in_specified_units(&self) -> f32 {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_in_specified_units_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_in_specified_units_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_in_specified_units_SVGLength(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_in_specified_units_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgLength as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `valueInSpecifiedUnits` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn set_value_in_specified_units(&self, value_in_specified_units: f32) {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_in_specified_units_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_in_specified_units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_in_specified_units_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value_in_specified_units =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_in_specified_units);
                __widl_f_set_value_in_specified_units_SVGLength(self_, value_in_specified_units)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_as_string_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgLength as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsString` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn value_as_string(&self) -> String {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_as_string_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_as_string_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_as_string_SVGLength(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgLength",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_as_string_SVGLength() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgLength as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgLength {
    #[cfg(all(feature = "SvgLength",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsString` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)\n\n*This API requires the following crate features to be activated: `SvgLength`*"]
    #[allow(clippy::all)]
    pub fn set_value_as_string(&self, value_as_string: &str) {
        #[cfg(all(feature = "SvgLength",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_as_string_SVGLength(
                self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_as_string: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_as_string_SVGLength(
            self_: <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgLength as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value_as_string =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_as_string);
                __widl_f_set_value_as_string_SVGLength(self_, value_as_string)
            };
            ()
        }
    }
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_NUMBER: u16 = 1u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_PERCENTAGE: u16 = 2u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_EMS: u16 = 3u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_EXS: u16 = 4u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_PX: u16 = 5u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_CM: u16 = 6u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_MM: u16 = 7u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_IN: u16 = 8u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_PT: u16 = 9u64 as u16;
}
impl SvgLength {
    pub const SVG_LENGTHTYPE_PC: u16 = 10u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_203006d4ff9525b3: [u8; 1064usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE6\x03\0\0\0\0\n\0\0\x02\tSVGLength\x1B__widl_instanceof_SVGLength\0\0\0\0-__widl_f_convert_to_specified_units_SVGLength\x01\0\0\x01\tSVGLength\x01\0\0\x01\x02\x05self_\tunit_type\x17convertToSpecifiedUnits\0\0\0,__widl_f_new_value_specified_units_SVGLength\x01\0\0\x01\tSVGLength\x01\0\0\x01\x03\x05self_\tunit_type\x18value_in_specified_units\x16newValueSpecifiedUnits\0\0\0\x1C__widl_f_unit_type_SVGLength\0\0\0\x01\tSVGLength\x01\0\x01\x08unitType\x01\x01\x05self_\x08unitType\0\0\0\x18__widl_f_value_SVGLength\x01\0\0\x01\tSVGLength\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\x1C__widl_f_set_value_SVGLength\x01\0\0\x01\tSVGLength\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0+__widl_f_value_in_specified_units_SVGLength\0\0\0\x01\tSVGLength\x01\0\x01\x15valueInSpecifiedUnits\x01\x01\x05self_\x15valueInSpecifiedUnits\0\0\0/__widl_f_set_value_in_specified_units_SVGLength\0\0\0\x01\tSVGLength\x01\0\x02\x15valueInSpecifiedUnits\x01\x02\x05self_\x18value_in_specified_units\x15valueInSpecifiedUnits\0\0\0\"__widl_f_value_as_string_SVGLength\0\0\0\x01\tSVGLength\x01\0\x01\rvalueAsString\x01\x01\x05self_\rvalueAsString\0\0\0&__widl_f_set_value_as_string_SVGLength\0\0\0\x01\tSVGLength\x01\0\x02\rvalueAsString\x01\x02\x05self_\x0Fvalue_as_string\rvalueAsString\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
