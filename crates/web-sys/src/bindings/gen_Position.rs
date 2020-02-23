use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Position` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)\n\n*This API requires the following crate features to be activated: `Position`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Position {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Position: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Position {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(80u32);
            inform(111u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Position {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Position {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Position {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Position {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Position {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Position {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Position {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Position {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Position {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Position>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Position {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Position {
        #[inline]
        fn from(obj: JsValue) -> Position {
            Position { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Position {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Position> for Position {
        #[inline]
        fn as_ref(&self) -> &Position {
            self
        }
    }
    impl From<Position> for JsValue {
        #[inline]
        fn from(obj: Position) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Position {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Position(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Position(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Position(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Position { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Position) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Position> for ::js_sys::Object {
    #[inline]
    fn from(obj: Position) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Position {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Coordinates", feature = "Position",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_coords_Position() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Position as WasmDescribe>::describe();
    <Coordinates as WasmDescribe>::describe();
}
impl Position {
    #[cfg(all(feature = "Coordinates", feature = "Position",))]
    #[allow(bad_style)]
    #[doc = "The `coords` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/coords)\n\n*This API requires the following crate features to be activated: `Coordinates`, `Position`*"]
    #[allow(clippy::all)]
    pub fn coords(&self) -> Coordinates {
        #[cfg(all(feature = "Coordinates", feature = "Position",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_coords_Position(
                self_: <&Position as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Coordinates as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_coords_Position(
            self_: <&Position as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Coordinates as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Position as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_coords_Position(self_)
            };
            <Coordinates as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Position",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timestamp_Position() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Position as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Position {
    #[cfg(all(feature = "Position",))]
    #[allow(bad_style)]
    #[doc = "The `timestamp` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/timestamp)\n\n*This API requires the following crate features to be activated: `Position`*"]
    #[allow(clippy::all)]
    pub fn timestamp(&self) -> f64 {
        #[cfg(all(feature = "Position",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timestamp_Position(
                self_: <&Position as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timestamp_Position(
            self_: <&Position as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Position as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timestamp_Position(self_)
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
pub static __WASM_BINDGEN_GENERATED_96618889ec7cb456: [u8; 286usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDC\0\0\0\0\0\x03\0\0\x02\x08Position\x1A__widl_instanceof_Position\0\0\0\0\x18__widl_f_coords_Position\0\0\0\x01\x08Position\x01\0\x01\x06coords\x01\x01\x05self_\x06coords\0\0\0\x1B__widl_f_timestamp_Position\0\0\0\x01\x08Position\x01\0\x01\ttimestamp\x01\x01\x05self_\ttimestamp\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
