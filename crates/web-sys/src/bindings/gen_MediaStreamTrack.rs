use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaStreamTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaStreamTrack {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaStreamTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaStreamTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(109u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for MediaStreamTrack {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaStreamTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaStreamTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaStreamTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaStreamTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaStreamTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaStreamTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaStreamTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaStreamTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaStreamTrack {
        #[inline]
        fn from(obj: JsValue) -> MediaStreamTrack {
            MediaStreamTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaStreamTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaStreamTrack> for MediaStreamTrack {
        #[inline]
        fn as_ref(&self) -> &MediaStreamTrack {
            self
        }
    }
    impl From<MediaStreamTrack> for JsValue {
        #[inline]
        fn from(obj: MediaStreamTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaStreamTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaStreamTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaStreamTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaStreamTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaStreamTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaStreamTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaStreamTrack> for EventTarget {
    #[inline]
    fn from(obj: MediaStreamTrack) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaStreamTrack {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaStreamTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaStreamTrack {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_apply_constraints_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `applyConstraints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn apply_constraints(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_apply_constraints_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_apply_constraints_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_apply_constraints_MediaStreamTrack(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_apply_constraints_with_constraints_MediaStreamTrack()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <&MediaTrackConstraints as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
    #[allow(bad_style)]
    #[doc = "The `applyConstraints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*"]
    #[allow(clippy::all)]
    pub fn apply_constraints_with_constraints(
        &self,
        constraints: &MediaTrackConstraints,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_apply_constraints_with_constraints_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                constraints: <&MediaTrackConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_apply_constraints_with_constraints_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            constraints: <&MediaTrackConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let constraints =
                    <&MediaTrackConstraints as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        constraints,
                    );
                __widl_f_apply_constraints_with_constraints_MediaStreamTrack(self_, constraints)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <MediaStreamTrack as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `clone()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/clone)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn clone(&self) -> MediaStreamTrack {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_MediaStreamTrack(self_)
            };
            <MediaStreamTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_constraints_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <MediaTrackConstraints as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
    #[allow(bad_style)]
    #[doc = "The `getConstraints()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getConstraints)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackConstraints`*"]
    #[allow(clippy::all)]
    pub fn get_constraints(&self) -> MediaTrackConstraints {
        #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackConstraints",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_constraints_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaTrackConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_constraints_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaTrackConstraints as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_constraints_MediaStreamTrack(self_)
            };
            <MediaTrackConstraints as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackSettings",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_settings_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <MediaTrackSettings as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackSettings",))]
    #[allow(bad_style)]
    #[doc = "The `getSettings()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSettings)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaTrackSettings`*"]
    #[allow(clippy::all)]
    pub fn get_settings(&self) -> MediaTrackSettings {
        #[cfg(all(feature = "MediaStreamTrack", feature = "MediaTrackSettings",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_settings_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaTrackSettings as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_settings_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaTrackSettings as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_settings_MediaStreamTrack(self_)
            };
            <MediaTrackSettings as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/stop)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_MediaStreamTrack(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/kind)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> String {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_MediaStreamTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/id)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_MediaStreamTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/label)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_MediaStreamTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enabled_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `enabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn enabled(&self) -> bool {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enabled_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enabled_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_enabled_MediaStreamTrack(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_enabled_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `enabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn set_enabled(&self, enabled: bool) {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_enabled_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_enabled_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(enabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let enabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(enabled);
                __widl_f_set_enabled_MediaStreamTrack(self_, enabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_muted_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `muted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/muted)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn muted(&self) -> bool {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_muted_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_muted_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_muted_MediaStreamTrack(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmute_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onmute` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn onmute(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmute_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmute_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmute_MediaStreamTrack(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmute_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onmute` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn set_onmute(&self, onmute: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmute_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmute: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmute_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmute: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmute);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmute =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmute,
                    );
                __widl_f_set_onmute_MediaStreamTrack(self_, onmute)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onunmute_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onunmute` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn onunmute(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onunmute_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onunmute_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onunmute_MediaStreamTrack(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onunmute_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onunmute` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn set_onunmute(&self, onunmute: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onunmute_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onunmute: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onunmute_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onunmute: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onunmute);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onunmute =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onunmute,
                    );
                __widl_f_set_onunmute_MediaStreamTrack(self_, onunmute)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <MediaStreamTrackState as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackState",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/readyState)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackState`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> MediaStreamTrackState {
        #[cfg(all(feature = "MediaStreamTrack", feature = "MediaStreamTrackState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamTrackState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamTrackState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_MediaStreamTrack(self_)
            };
            <MediaStreamTrackState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_MediaStreamTrack(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_MediaStreamTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStreamTrack as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStreamTrack {
    #[cfg(all(feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_MediaStreamTrack(
                self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_MediaStreamTrack(
            self_: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_MediaStreamTrack(self_, onended)
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
pub static __WASM_BINDGEN_GENERATED_a488fd66d9f30765: [u8; 1831usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE5\x06\0\0\0\0\x14\0\0\x02\x10MediaStreamTrack\"__widl_instanceof_MediaStreamTrack\0\0\0\0+__widl_f_apply_constraints_MediaStreamTrack\x01\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x01\x05self_\x10applyConstraints\0\0\0<__widl_f_apply_constraints_with_constraints_MediaStreamTrack\x01\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x02\x05self_\x0Bconstraints\x10applyConstraints\0\0\0\x1F__widl_f_clone_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x01\x05self_\x05clone\0\0\0)__widl_f_get_constraints_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x01\x05self_\x0EgetConstraints\0\0\0&__widl_f_get_settings_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x01\x05self_\x0BgetSettings\0\0\0\x1E__widl_f_stop_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\0\x01\x01\x05self_\x04stop\0\0\0\x1E__widl_f_kind_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x1C__widl_f_id_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x1F__widl_f_label_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0!__widl_f_enabled_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x07enabled\x01\x01\x05self_\x07enabled\0\0\0%__widl_f_set_enabled_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x02\x07enabled\x01\x02\x05self_\x07enabled\x07enabled\0\0\0\x1F__widl_f_muted_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x05muted\x01\x01\x05self_\x05muted\0\0\0 __widl_f_onmute_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x06onmute\x01\x01\x05self_\x06onmute\0\0\0$__widl_f_set_onmute_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x02\x06onmute\x01\x02\x05self_\x06onmute\x06onmute\0\0\0\"__widl_f_onunmute_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x08onunmute\x01\x01\x05self_\x08onunmute\0\0\0&__widl_f_set_onunmute_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x02\x08onunmute\x01\x02\x05self_\x08onunmute\x08onunmute\0\0\0%__widl_f_ready_state_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0!__widl_f_onended_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0%__widl_f_set_onended_MediaStreamTrack\0\0\0\x01\x10MediaStreamTrack\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
