use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Gamepad` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Gamepad {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Gamepad: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Gamepad {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
        }
    }
    impl core::ops::Deref for Gamepad {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Gamepad {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Gamepad {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Gamepad {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Gamepad {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Gamepad {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Gamepad {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Gamepad {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Gamepad {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Gamepad>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Gamepad {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Gamepad {
        #[inline]
        fn from(obj: JsValue) -> Gamepad {
            Gamepad { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Gamepad {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Gamepad> for Gamepad {
        #[inline]
        fn as_ref(&self) -> &Gamepad {
            self
        }
    }
    impl From<Gamepad> for JsValue {
        #[inline]
        fn from(obj: Gamepad) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Gamepad {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Gamepad(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Gamepad(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Gamepad(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Gamepad { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Gamepad) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Gamepad> for ::js_sys::Object {
    #[inline]
    fn from(obj: Gamepad) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Gamepad {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_Gamepad(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_index_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `index` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn index(&self) -> u32 {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_index_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_index_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_index_Gamepad(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad", feature = "GamepadMappingType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mapping_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <GamepadMappingType as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad", feature = "GamepadMappingType",))]
    #[allow(bad_style)]
    #[doc = "The `mapping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadMappingType`*"]
    #[allow(clippy::all)]
    pub fn mapping(&self) -> GamepadMappingType {
        #[cfg(all(feature = "Gamepad", feature = "GamepadMappingType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mapping_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mapping_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mapping_Gamepad(self_)
            };
            <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad", feature = "GamepadHand",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hand_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <GamepadHand as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad", feature = "GamepadHand",))]
    #[allow(bad_style)]
    #[doc = "The `hand` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadHand`*"]
    #[allow(clippy::all)]
    pub fn hand(&self) -> GamepadHand {
        #[cfg(all(feature = "Gamepad", feature = "GamepadHand",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hand_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hand_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hand_Gamepad(self_)
            };
            <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_display_id_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `displayId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/displayId)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn display_id(&self) -> u32 {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_display_id_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_display_id_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_display_id_Gamepad(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connected_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `connected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn connected(&self) -> bool {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connected_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connected_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connected_Gamepad(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buttons_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `buttons` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn buttons(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buttons_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buttons_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buttons_Gamepad(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_axes_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `axes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn axes(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_axes_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_axes_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_axes_Gamepad(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timestamp_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `timestamp` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn timestamp(&self) -> f64 {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timestamp_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timestamp_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timestamp_Gamepad(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad", feature = "GamepadPose",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pose_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <Option<GamepadPose> as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad", feature = "GamepadPose",))]
    #[allow(bad_style)]
    #[doc = "The `pose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadPose`*"]
    #[allow(clippy::all)]
    pub fn pose(&self) -> Option<GamepadPose> {
        #[cfg(all(feature = "Gamepad", feature = "GamepadPose",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pose_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<GamepadPose> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pose_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<GamepadPose> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pose_Gamepad(self_)
            };
            <Option<GamepadPose> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Gamepad",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_haptic_actuators_Gamepad() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Gamepad as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Gamepad {
    #[cfg(all(feature = "Gamepad",))]
    #[allow(bad_style)]
    #[doc = "The `hapticActuators` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    #[allow(clippy::all)]
    pub fn haptic_actuators(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Gamepad",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_haptic_actuators_Gamepad(
                self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_haptic_actuators_Gamepad(
            self_: <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Gamepad as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_haptic_actuators_Gamepad(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_83627ed302bcd3de: [u8; 876usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\x03\0\0\0\0\x0C\0\0\x02\x07Gamepad\x19__widl_instanceof_Gamepad\0\0\0\0\x13__widl_f_id_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x16__widl_f_index_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x05index\x01\x01\x05self_\x05index\0\0\0\x18__widl_f_mapping_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x07mapping\x01\x01\x05self_\x07mapping\0\0\0\x15__widl_f_hand_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x04hand\x01\x01\x05self_\x04hand\0\0\0\x1B__widl_f_display_id_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\tdisplayId\x01\x01\x05self_\tdisplayId\0\0\0\x1A__widl_f_connected_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\tconnected\x01\x01\x05self_\tconnected\0\0\0\x18__widl_f_buttons_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x07buttons\x01\x01\x05self_\x07buttons\0\0\0\x15__widl_f_axes_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x04axes\x01\x01\x05self_\x04axes\0\0\0\x1A__widl_f_timestamp_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\ttimestamp\x01\x01\x05self_\ttimestamp\0\0\0\x15__widl_f_pose_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x04pose\x01\x01\x05self_\x04pose\0\0\0!__widl_f_haptic_actuators_Gamepad\0\0\0\x01\x07Gamepad\x01\0\x01\x0FhapticActuators\x01\x01\x05self_\x0FhapticActuators\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
