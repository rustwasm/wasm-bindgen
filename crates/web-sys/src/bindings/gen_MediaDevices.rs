use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaDevices` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaDevices {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaDevices: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaDevices {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for MediaDevices {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaDevices {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaDevices {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaDevices {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaDevices {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaDevices {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaDevices {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaDevices {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaDevices {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaDevices>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaDevices {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaDevices {
        #[inline]
        fn from(obj: JsValue) -> MediaDevices {
            MediaDevices { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaDevices {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaDevices> for MediaDevices {
        #[inline]
        fn as_ref(&self) -> &MediaDevices {
            self
        }
    }
    impl From<MediaDevices> for JsValue {
        #[inline]
        fn from(obj: MediaDevices) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaDevices {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaDevices(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaDevices(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaDevices(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaDevices { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaDevices) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaDevices> for EventTarget {
    #[inline]
    fn from(obj: MediaDevices) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaDevices {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaDevices> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaDevices) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaDevices {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaDevices",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enumerate_devices_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDevices as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices",))]
    #[allow(bad_style)]
    #[doc = "The `enumerateDevices()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    #[allow(clippy::all)]
    pub fn enumerate_devices(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaDevices",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enumerate_devices_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enumerate_devices_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_enumerate_devices_MediaDevices(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaDevices", feature = "MediaTrackSupportedConstraints",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_supported_constraints_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDevices as WasmDescribe>::describe();
    <MediaTrackSupportedConstraints as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices", feature = "MediaTrackSupportedConstraints",))]
    #[allow(bad_style)]
    #[doc = "The `getSupportedConstraints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `MediaTrackSupportedConstraints`*"]
    #[allow(clippy::all)]
    pub fn get_supported_constraints(&self) -> MediaTrackSupportedConstraints {
        #[cfg(all(feature = "MediaDevices", feature = "MediaTrackSupportedConstraints",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_supported_constraints_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaTrackSupportedConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_supported_constraints_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaTrackSupportedConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_supported_constraints_MediaDevices(self_)
            };
            <MediaTrackSupportedConstraints as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDevices",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_user_media_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDevices as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices",))]
    #[allow(bad_style)]
    #[doc = "The `getUserMedia()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    #[allow(clippy::all)]
    pub fn get_user_media(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaDevices",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_user_media_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_user_media_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_user_media_MediaDevices(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaDevices", feature = "MediaStreamConstraints",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_user_media_with_constraints_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaDevices as WasmDescribe>::describe();
    <&MediaStreamConstraints as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices", feature = "MediaStreamConstraints",))]
    #[allow(bad_style)]
    #[doc = "The `getUserMedia()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `MediaStreamConstraints`*"]
    #[allow(clippy::all)]
    pub fn get_user_media_with_constraints(
        &self,
        constraints: &MediaStreamConstraints,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaDevices", feature = "MediaStreamConstraints",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_user_media_with_constraints_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                constraints: <&MediaStreamConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_user_media_with_constraints_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            constraints: <&MediaStreamConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(constraints);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let constraints =
                    <&MediaStreamConstraints as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        constraints,
                    );
                __widl_f_get_user_media_with_constraints_MediaDevices(self_, constraints)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaDevices",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondevicechange_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaDevices as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices",))]
    #[allow(bad_style)]
    #[doc = "The `ondevicechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    #[allow(clippy::all)]
    pub fn ondevicechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaDevices",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondevicechange_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondevicechange_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondevicechange_MediaDevices(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDevices",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondevicechange_MediaDevices() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaDevices as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaDevices {
    #[cfg(all(feature = "MediaDevices",))]
    #[allow(bad_style)]
    #[doc = "The `ondevicechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    #[allow(clippy::all)]
    pub fn set_ondevicechange(&self, ondevicechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaDevices",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondevicechange_MediaDevices(
                self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondevicechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondevicechange_MediaDevices(
            self_: <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondevicechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondevicechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaDevices as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondevicechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondevicechange,
                    );
                __widl_f_set_ondevicechange_MediaDevices(self_, ondevicechange)
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
pub static __WASM_BINDGEN_GENERATED_84ef06a45f887570: [u8; 750usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAC\x02\0\0\0\0\x07\0\0\x02\x0CMediaDevices\x1E__widl_instanceof_MediaDevices\0\0\0\0'__widl_f_enumerate_devices_MediaDevices\x01\0\0\x01\x0CMediaDevices\x01\0\0\x01\x01\x05self_\x10enumerateDevices\0\0\0/__widl_f_get_supported_constraints_MediaDevices\0\0\0\x01\x0CMediaDevices\x01\0\0\x01\x01\x05self_\x17getSupportedConstraints\0\0\0$__widl_f_get_user_media_MediaDevices\x01\0\0\x01\x0CMediaDevices\x01\0\0\x01\x01\x05self_\x0CgetUserMedia\0\0\05__widl_f_get_user_media_with_constraints_MediaDevices\x01\0\0\x01\x0CMediaDevices\x01\0\0\x01\x02\x05self_\x0Bconstraints\x0CgetUserMedia\0\0\0$__widl_f_ondevicechange_MediaDevices\0\0\0\x01\x0CMediaDevices\x01\0\x01\x0Eondevicechange\x01\x01\x05self_\x0Eondevicechange\0\0\0(__widl_f_set_ondevicechange_MediaDevices\0\0\0\x01\x0CMediaDevices\x01\0\x02\x0Eondevicechange\x01\x02\x05self_\x0Eondevicechange\x0Eondevicechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
