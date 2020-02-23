use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VTTRegion` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VttRegion {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VttRegion: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VttRegion {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(86u32);
            inform(84u32);
            inform(84u32);
            inform(82u32);
            inform(101u32);
            inform(103u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for VttRegion {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VttRegion {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VttRegion {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VttRegion {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VttRegion {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VttRegion {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VttRegion {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VttRegion {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VttRegion {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VttRegion>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VttRegion {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VttRegion {
        #[inline]
        fn from(obj: JsValue) -> VttRegion {
            VttRegion { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VttRegion {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VttRegion> for VttRegion {
        #[inline]
        fn as_ref(&self) -> &VttRegion {
            self
        }
    }
    impl From<VttRegion> for JsValue {
        #[inline]
        fn from(obj: VttRegion) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VttRegion {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VTTRegion(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VTTRegion(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VTTRegion(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VttRegion { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VttRegion) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VttRegion> for ::js_sys::Object {
    #[inline]
    fn from(obj: VttRegion) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VttRegion {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <VttRegion as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `new VTTRegion(..)` constructor, creating a new instance of `VTTRegion`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/VTTRegion)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<VttRegion, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_VTTRegion() -> <VttRegion as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_VTTRegion() -> <VttRegion as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_VTTRegion() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<VttRegion as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_VTTRegion(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_id_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `id` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/id)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_id(&self, id: &str) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_id_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_id_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_set_id_VTTRegion(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> f64 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_VTTRegion(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/width)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: f64) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_VTTRegion(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lines_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `lines` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn lines(&self) -> i32 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lines_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lines_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lines_VTTRegion(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_lines_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `lines` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/lines)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_lines(&self, lines: i32) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_lines_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lines: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_lines_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lines: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(lines);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let lines = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lines);
                __widl_f_set_lines_VTTRegion(self_, lines)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_region_anchor_x_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `regionAnchorX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn region_anchor_x(&self) -> f64 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_region_anchor_x_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_region_anchor_x_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_region_anchor_x_VTTRegion(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_region_anchor_x_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `regionAnchorX` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_region_anchor_x(&self, region_anchor_x: f64) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_region_anchor_x_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                region_anchor_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_region_anchor_x_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            region_anchor_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(region_anchor_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let region_anchor_x =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(region_anchor_x);
                __widl_f_set_region_anchor_x_VTTRegion(self_, region_anchor_x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_region_anchor_y_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `regionAnchorY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn region_anchor_y(&self) -> f64 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_region_anchor_y_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_region_anchor_y_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_region_anchor_y_VTTRegion(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_region_anchor_y_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `regionAnchorY` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/regionAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_region_anchor_y(&self, region_anchor_y: f64) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_region_anchor_y_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                region_anchor_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_region_anchor_y_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            region_anchor_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(region_anchor_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let region_anchor_y =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(region_anchor_y);
                __widl_f_set_region_anchor_y_VTTRegion(self_, region_anchor_y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_viewport_anchor_x_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `viewportAnchorX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn viewport_anchor_x(&self) -> f64 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_viewport_anchor_x_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_viewport_anchor_x_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_viewport_anchor_x_VTTRegion(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_viewport_anchor_x_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `viewportAnchorX` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorX)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_viewport_anchor_x(&self, viewport_anchor_x: f64) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_viewport_anchor_x_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                viewport_anchor_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_viewport_anchor_x_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            viewport_anchor_x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(viewport_anchor_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let viewport_anchor_x =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(viewport_anchor_x);
                __widl_f_set_viewport_anchor_x_VTTRegion(self_, viewport_anchor_x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_viewport_anchor_y_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `viewportAnchorY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn viewport_anchor_y(&self) -> f64 {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_viewport_anchor_y_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_viewport_anchor_y_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_viewport_anchor_y_VTTRegion(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_viewport_anchor_y_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `viewportAnchorY` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/viewportAnchorY)\n\n*This API requires the following crate features to be activated: `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_viewport_anchor_y(&self, viewport_anchor_y: f64) {
        #[cfg(all(feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_viewport_anchor_y_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                viewport_anchor_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_viewport_anchor_y_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            viewport_anchor_y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(viewport_anchor_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let viewport_anchor_y =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(viewport_anchor_y);
                __widl_f_set_viewport_anchor_y_VTTRegion(self_, viewport_anchor_y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scroll_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttRegion as WasmDescribe>::describe();
    <ScrollSetting as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `scroll` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)\n\n*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn scroll(&self) -> ScrollSetting {
        #[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scroll_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScrollSetting as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scroll_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScrollSetting as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scroll_VTTRegion(self_)
            };
            <ScrollSetting as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scroll_VTTRegion() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttRegion as WasmDescribe>::describe();
    <ScrollSetting as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttRegion {
    #[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `scroll` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTRegion/scroll)\n\n*This API requires the following crate features to be activated: `ScrollSetting`, `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_scroll(&self, scroll: ScrollSetting) {
        #[cfg(all(feature = "ScrollSetting", feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scroll_VTTRegion(
                self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scroll: <ScrollSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scroll_VTTRegion(
            self_: <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scroll: <ScrollSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scroll);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttRegion as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scroll =
                    <ScrollSetting as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scroll);
                __widl_f_set_scroll_VTTRegion(self_, scroll)
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
pub static __WASM_BINDGEN_GENERATED_1ded2b01f95eab29: [u8; 1576usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE6\x05\0\0\0\0\x12\0\0\x02\tVTTRegion\x1B__widl_instanceof_VTTRegion\0\0\0\0\x16__widl_f_new_VTTRegion\x01\0\0\x01\tVTTRegion\0\x01\0\x03new\0\0\0\x15__widl_f_id_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x19__widl_f_set_id_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x02id\x01\x02\x05self_\x02id\x02id\0\0\0\x18__widl_f_width_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1C__widl_f_set_width_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\x18__widl_f_lines_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x05lines\x01\x01\x05self_\x05lines\0\0\0\x1C__widl_f_set_lines_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x05lines\x01\x02\x05self_\x05lines\x05lines\0\0\0\"__widl_f_region_anchor_x_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\rregionAnchorX\x01\x01\x05self_\rregionAnchorX\0\0\0&__widl_f_set_region_anchor_x_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\rregionAnchorX\x01\x02\x05self_\x0Fregion_anchor_x\rregionAnchorX\0\0\0\"__widl_f_region_anchor_y_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\rregionAnchorY\x01\x01\x05self_\rregionAnchorY\0\0\0&__widl_f_set_region_anchor_y_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\rregionAnchorY\x01\x02\x05self_\x0Fregion_anchor_y\rregionAnchorY\0\0\0$__widl_f_viewport_anchor_x_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x0FviewportAnchorX\x01\x01\x05self_\x0FviewportAnchorX\0\0\0(__widl_f_set_viewport_anchor_x_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x0FviewportAnchorX\x01\x02\x05self_\x11viewport_anchor_x\x0FviewportAnchorX\0\0\0$__widl_f_viewport_anchor_y_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x0FviewportAnchorY\x01\x01\x05self_\x0FviewportAnchorY\0\0\0(__widl_f_set_viewport_anchor_y_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x0FviewportAnchorY\x01\x02\x05self_\x11viewport_anchor_y\x0FviewportAnchorY\0\0\0\x19__widl_f_scroll_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x01\x06scroll\x01\x01\x05self_\x06scroll\0\0\0\x1D__widl_f_set_scroll_VTTRegion\0\0\0\x01\tVTTRegion\x01\0\x02\x06scroll\x01\x02\x05self_\x06scroll\x06scroll\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
