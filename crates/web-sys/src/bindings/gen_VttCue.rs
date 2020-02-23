use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VTTCue` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VttCue {
    obj: TextTrackCue,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VttCue: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VttCue {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(86u32);
            inform(84u32);
            inform(84u32);
            inform(67u32);
            inform(117u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for VttCue {
        type Target = TextTrackCue;
        #[inline]
        fn deref(&self) -> &TextTrackCue {
            &self.obj
        }
    }
    impl IntoWasmAbi for VttCue {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VttCue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VttCue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VttCue {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VttCue {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VttCue {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VttCue {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VttCue {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VttCue>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VttCue {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VttCue {
        #[inline]
        fn from(obj: JsValue) -> VttCue {
            VttCue { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VttCue {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VttCue> for VttCue {
        #[inline]
        fn as_ref(&self) -> &VttCue {
            self
        }
    }
    impl From<VttCue> for JsValue {
        #[inline]
        fn from(obj: VttCue) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VttCue {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VTTCue(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VTTCue(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VTTCue(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VttCue { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VttCue) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VttCue> for TextTrackCue {
    #[inline]
    fn from(obj: VttCue) -> TextTrackCue {
        use wasm_bindgen::JsCast;
        TextTrackCue::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<TextTrackCue> for VttCue {
    #[inline]
    fn as_ref(&self) -> &TextTrackCue {
        use wasm_bindgen::JsCast;
        TextTrackCue::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<VttCue> for EventTarget {
    #[inline]
    fn from(obj: VttCue) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for VttCue {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<VttCue> for ::js_sys::Object {
    #[inline]
    fn from(obj: VttCue) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VttCue {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <VttCue as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `new VTTCue(..)` constructor, creating a new instance of `VTTCue`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/VTTCue)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn new(
        start_time: f64,
        end_time: f64,
        text: &str,
    ) -> Result<VttCue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_VTTCue(
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VttCue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_VTTCue(
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VttCue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(start_time);
            drop(end_time);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                let end_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_time);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_new_VTTCue(start_time, end_time, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<VttCue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DocumentFragment", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_cue_as_html_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <DocumentFragment as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "DocumentFragment", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `getCueAsHTML()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn get_cue_as_html(&self) -> DocumentFragment {
        #[cfg(all(feature = "DocumentFragment", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_cue_as_html_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_cue_as_html_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_cue_as_html_VTTCue(self_)
            };
            <DocumentFragment as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue", feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_region_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <Option<VttRegion> as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue", feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `region` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)\n\n*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn region(&self) -> Option<VttRegion> {
        #[cfg(all(feature = "VttCue", feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_region_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<VttRegion> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_region_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<VttRegion> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_region_VTTCue(self_)
            };
            <Option<VttRegion> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue", feature = "VttRegion",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_region_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <Option<&VttRegion> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue", feature = "VttRegion",))]
    #[allow(bad_style)]
    #[doc = "The `region` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)\n\n*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
    #[allow(clippy::all)]
    pub fn set_region(&self, region: Option<&VttRegion>) {
        #[cfg(all(feature = "VttCue", feature = "VttRegion",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_region_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                region: <Option<&VttRegion> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_region_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            region: <Option<&VttRegion> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(region);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let region =
                    <Option<&VttRegion> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(region);
                __widl_f_set_region_VTTCue(self_, region)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertical_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <DirectionSetting as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `vertical` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)\n\n*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn vertical(&self) -> DirectionSetting {
        #[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertical_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DirectionSetting as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertical_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DirectionSetting as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_vertical_VTTCue(self_)
            };
            <DirectionSetting as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_vertical_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <DirectionSetting as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `vertical` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)\n\n*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_vertical(&self, vertical: DirectionSetting) {
        #[cfg(all(feature = "DirectionSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_vertical_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                vertical: <DirectionSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_vertical_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            vertical: <DirectionSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(vertical);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let vertical =
                    <DirectionSetting as wasm_bindgen::convert::IntoWasmAbi>::into_abi(vertical);
                __widl_f_set_vertical_VTTCue(self_, vertical)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_snap_to_lines_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `snapToLines` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn snap_to_lines(&self) -> bool {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_snap_to_lines_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_snap_to_lines_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_snap_to_lines_VTTCue(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_snap_to_lines_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `snapToLines` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_snap_to_lines(&self, snap_to_lines: bool) {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_snap_to_lines_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                snap_to_lines: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_snap_to_lines_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            snap_to_lines: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(snap_to_lines);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let snap_to_lines =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(snap_to_lines);
                __widl_f_set_snap_to_lines_VTTCue(self_, snap_to_lines)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `line` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn line(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_line_VTTCue(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `line` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_line(&self, line: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let line =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        line,
                    );
                __widl_f_set_line_VTTCue(self_, line)
            };
            ()
        }
    }
}
#[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <LineAlignSetting as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `lineAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)\n\n*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn line_align(&self) -> LineAlignSetting {
        #[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <LineAlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <LineAlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_line_align_VTTCue(self_)
            };
            <LineAlignSetting as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_line_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <LineAlignSetting as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `lineAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)\n\n*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_line_align(&self, line_align: LineAlignSetting) {
        #[cfg(all(feature = "LineAlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_line_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                line_align: <LineAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_line_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            line_align: <LineAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(line_align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let line_align =
                    <LineAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::into_abi(line_align);
                __widl_f_set_line_align_VTTCue(self_, line_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `position` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn position(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_VTTCue(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `position` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_position(&self, position: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                position: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            position: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(position);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let position =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        position,
                    );
                __widl_f_set_position_VTTCue(self_, position)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <PositionAlignSetting as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `positionAlign` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)\n\n*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn position_align(&self) -> PositionAlignSetting {
        #[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PositionAlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PositionAlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_align_VTTCue(self_)
            };
            <PositionAlignSetting as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <PositionAlignSetting as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `positionAlign` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)\n\n*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_position_align(&self, position_align: PositionAlignSetting) {
        #[cfg(all(feature = "PositionAlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                position_align: <PositionAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            position_align: <PositionAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(position_align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let position_align =
                    <PositionAlignSetting as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        position_align,
                    );
                __widl_f_set_position_align_VTTCue(self_, position_align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> f64 {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_VTTCue(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_size_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `size` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_size(&self, size: f64) {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_size_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_size_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_set_size_VTTCue(self_, size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <AlignSetting as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)\n\n*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> AlignSetting {
        #[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AlignSetting as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_VTTCue(self_)
            };
            <AlignSetting as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <AlignSetting as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)\n\n*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: AlignSetting) {
        #[cfg(all(feature = "AlignSetting", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <AlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <AlignSetting as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <AlignSetting as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_VTTCue(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VttCue as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> String {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_VTTCue(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_VTTCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VttCue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VttCue {
    #[cfg(all(feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)\n\n*This API requires the following crate features to be activated: `VttCue`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) {
        #[cfg(all(feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_VTTCue(
                self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_VTTCue(
            self_: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_VTTCue(self_, text)
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
pub static __WASM_BINDGEN_GENERATED_e60ddf885ab62d86: [u8; 1723usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}y\x06\0\0\0\0\x17\0\0\x02\x06VTTCue\x18__widl_instanceof_VTTCue\0\0\0\0\x13__widl_f_new_VTTCue\x01\0\0\x01\x06VTTCue\0\x01\x03\nstart_time\x08end_time\x04text\x03new\0\0\0\x1F__widl_f_get_cue_as_html_VTTCue\0\0\0\x01\x06VTTCue\x01\0\0\x01\x01\x05self_\x0CgetCueAsHTML\0\0\0\x16__widl_f_region_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x06region\x01\x01\x05self_\x06region\0\0\0\x1A__widl_f_set_region_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x06region\x01\x02\x05self_\x06region\x06region\0\0\0\x18__widl_f_vertical_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x08vertical\x01\x01\x05self_\x08vertical\0\0\0\x1C__widl_f_set_vertical_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x08vertical\x01\x02\x05self_\x08vertical\x08vertical\0\0\0\x1D__widl_f_snap_to_lines_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x0BsnapToLines\x01\x01\x05self_\x0BsnapToLines\0\0\0!__widl_f_set_snap_to_lines_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x0BsnapToLines\x01\x02\x05self_\rsnap_to_lines\x0BsnapToLines\0\0\0\x14__widl_f_line_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x04line\x01\x01\x05self_\x04line\0\0\0\x18__widl_f_set_line_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x04line\x01\x02\x05self_\x04line\x04line\0\0\0\x1A__widl_f_line_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\tlineAlign\x01\x01\x05self_\tlineAlign\0\0\0\x1E__widl_f_set_line_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\tlineAlign\x01\x02\x05self_\nline_align\tlineAlign\0\0\0\x18__widl_f_position_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x08position\x01\x01\x05self_\x08position\0\0\0\x1C__widl_f_set_position_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x08position\x01\x02\x05self_\x08position\x08position\0\0\0\x1E__widl_f_position_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\rpositionAlign\x01\x01\x05self_\rpositionAlign\0\0\0\"__widl_f_set_position_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\rpositionAlign\x01\x02\x05self_\x0Eposition_align\rpositionAlign\0\0\0\x14__widl_f_size_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\x18__widl_f_set_size_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x04size\x01\x02\x05self_\x04size\x04size\0\0\0\x15__widl_f_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0\x19__widl_f_set_align_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\x14__widl_f_text_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0\x18__widl_f_set_text_VTTCue\0\0\0\x01\x06VTTCue\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
