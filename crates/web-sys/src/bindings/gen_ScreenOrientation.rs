use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ScreenOrientation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ScreenOrientation {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ScreenOrientation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ScreenOrientation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(110u32);
            inform(79u32);
            inform(114u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for ScreenOrientation {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for ScreenOrientation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ScreenOrientation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ScreenOrientation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ScreenOrientation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ScreenOrientation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ScreenOrientation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ScreenOrientation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ScreenOrientation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ScreenOrientation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ScreenOrientation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ScreenOrientation {
        #[inline]
        fn from(obj: JsValue) -> ScreenOrientation {
            ScreenOrientation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ScreenOrientation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ScreenOrientation> for ScreenOrientation {
        #[inline]
        fn as_ref(&self) -> &ScreenOrientation {
            self
        }
    }
    impl From<ScreenOrientation> for JsValue {
        #[inline]
        fn from(obj: ScreenOrientation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ScreenOrientation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ScreenOrientation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ScreenOrientation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ScreenOrientation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ScreenOrientation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ScreenOrientation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ScreenOrientation> for EventTarget {
    #[inline]
    fn from(obj: ScreenOrientation) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ScreenOrientation {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ScreenOrientation> for ::js_sys::Object {
    #[inline]
    fn from(obj: ScreenOrientation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ScreenOrientation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "OrientationLockType", feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lock_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <OrientationLockType as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "OrientationLockType", feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `lock()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)\n\n*This API requires the following crate features to be activated: `OrientationLockType`, `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn lock(
        &self,
        orientation: OrientationLockType,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OrientationLockType", feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lock_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                orientation: <OrientationLockType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lock_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            orientation: <OrientationLockType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(orientation);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let orientation =
                    <OrientationLockType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        orientation,
                    );
                __widl_f_lock_ScreenOrientation(self_, orientation)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unlock_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `unlock()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn unlock(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unlock_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unlock_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_unlock_ScreenOrientation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OrientationType", feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <OrientationType as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "OrientationType", feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)\n\n*This API requires the following crate features to be activated: `OrientationType`, `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> Result<OrientationType, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OrientationType", feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OrientationType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OrientationType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_ScreenOrientation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OrientationType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_angle_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `angle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn angle(&self) -> Result<u16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_angle_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_angle_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_angle_ScreenOrientation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_ScreenOrientation(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScreenOrientation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_ScreenOrientation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScreenOrientation as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScreenOrientation {
    #[cfg(all(feature = "ScreenOrientation",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ScreenOrientation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_ScreenOrientation(
                self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_ScreenOrientation(
            self_: <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScreenOrientation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_ScreenOrientation(self_, onchange)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0879a32d774a904c: [u8; 677usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}c\x02\0\0\0\0\x07\0\0\x02\x11ScreenOrientation#__widl_instanceof_ScreenOrientation\0\0\0\0\x1F__widl_f_lock_ScreenOrientation\x01\0\0\x01\x11ScreenOrientation\x01\0\0\x01\x02\x05self_\x0Borientation\x04lock\0\0\0!__widl_f_unlock_ScreenOrientation\x01\0\0\x01\x11ScreenOrientation\x01\0\0\x01\x01\x05self_\x06unlock\0\0\0\x1F__widl_f_type_ScreenOrientation\x01\0\0\x01\x11ScreenOrientation\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0 __widl_f_angle_ScreenOrientation\x01\0\0\x01\x11ScreenOrientation\x01\0\x01\x05angle\x01\x01\x05self_\x05angle\0\0\0#__widl_f_onchange_ScreenOrientation\0\0\0\x01\x11ScreenOrientation\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0'__widl_f_set_onchange_ScreenOrientation\0\0\0\x01\x11ScreenOrientation\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
