use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AnimationEffect` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect)\n\n*This API requires the following crate features to be activated: `AnimationEffect`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AnimationEffect {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AnimationEffect: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AnimationEffect {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for AnimationEffect {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AnimationEffect {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AnimationEffect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AnimationEffect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AnimationEffect {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AnimationEffect {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AnimationEffect {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AnimationEffect {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AnimationEffect {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AnimationEffect>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AnimationEffect {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AnimationEffect {
        #[inline]
        fn from(obj: JsValue) -> AnimationEffect {
            AnimationEffect { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AnimationEffect {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AnimationEffect> for AnimationEffect {
        #[inline]
        fn as_ref(&self) -> &AnimationEffect {
            self
        }
    }
    impl From<AnimationEffect> for JsValue {
        #[inline]
        fn from(obj: AnimationEffect) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AnimationEffect {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AnimationEffect(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AnimationEffect(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AnimationEffect(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AnimationEffect { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AnimationEffect) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AnimationEffect> for ::js_sys::Object {
    #[inline]
    fn from(obj: AnimationEffect) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AnimationEffect {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AnimationEffect", feature = "ComputedEffectTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_computed_timing_AnimationEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEffect as WasmDescribe>::describe();
    <ComputedEffectTiming as WasmDescribe>::describe();
}
impl AnimationEffect {
    #[cfg(all(feature = "AnimationEffect", feature = "ComputedEffectTiming",))]
    #[allow(bad_style)]
    #[doc = "The `getComputedTiming()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getComputedTiming)\n\n*This API requires the following crate features to be activated: `AnimationEffect`, `ComputedEffectTiming`*"]
    #[allow(clippy::all)]
    pub fn get_computed_timing(&self) -> ComputedEffectTiming {
        #[cfg(all(feature = "AnimationEffect", feature = "ComputedEffectTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_computed_timing_AnimationEffect(
                self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ComputedEffectTiming as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_computed_timing_AnimationEffect(
            self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ComputedEffectTiming as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_computed_timing_AnimationEffect(self_)
            };
            <ComputedEffectTiming as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnimationEffect", feature = "EffectTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_timing_AnimationEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEffect as WasmDescribe>::describe();
    <EffectTiming as WasmDescribe>::describe();
}
impl AnimationEffect {
    #[cfg(all(feature = "AnimationEffect", feature = "EffectTiming",))]
    #[allow(bad_style)]
    #[doc = "The `getTiming()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getTiming)\n\n*This API requires the following crate features to be activated: `AnimationEffect`, `EffectTiming`*"]
    #[allow(clippy::all)]
    pub fn get_timing(&self) -> EffectTiming {
        #[cfg(all(feature = "AnimationEffect", feature = "EffectTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_timing_AnimationEffect(
                self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <EffectTiming as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_timing_AnimationEffect(
            self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <EffectTiming as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_timing_AnimationEffect(self_)
            };
            <EffectTiming as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnimationEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_timing_AnimationEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEffect as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnimationEffect {
    #[cfg(all(feature = "AnimationEffect",))]
    #[allow(bad_style)]
    #[doc = "The `updateTiming()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)\n\n*This API requires the following crate features to be activated: `AnimationEffect`*"]
    #[allow(clippy::all)]
    pub fn update_timing(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnimationEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_timing_AnimationEffect(
                self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_timing_AnimationEffect(
            self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_update_timing_AnimationEffect(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AnimationEffect", feature = "OptionalEffectTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_timing_with_timing_AnimationEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnimationEffect as WasmDescribe>::describe();
    <&OptionalEffectTiming as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnimationEffect {
    #[cfg(all(feature = "AnimationEffect", feature = "OptionalEffectTiming",))]
    #[allow(bad_style)]
    #[doc = "The `updateTiming()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)\n\n*This API requires the following crate features to be activated: `AnimationEffect`, `OptionalEffectTiming`*"]
    #[allow(clippy::all)]
    pub fn update_timing_with_timing(
        &self,
        timing: &OptionalEffectTiming,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnimationEffect", feature = "OptionalEffectTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_timing_with_timing_AnimationEffect(
                self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timing: <&OptionalEffectTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_timing_with_timing_AnimationEffect(
            self_: <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timing: <&OptionalEffectTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(timing);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AnimationEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let timing =
                    <&OptionalEffectTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timing);
                __widl_f_update_timing_with_timing_AnimationEffect(self_, timing)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4a93721835be1b17: [u8; 527usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCD\x01\0\0\0\0\x05\0\0\x02\x0FAnimationEffect!__widl_instanceof_AnimationEffect\0\0\0\0,__widl_f_get_computed_timing_AnimationEffect\0\0\0\x01\x0FAnimationEffect\x01\0\0\x01\x01\x05self_\x11getComputedTiming\0\0\0#__widl_f_get_timing_AnimationEffect\0\0\0\x01\x0FAnimationEffect\x01\0\0\x01\x01\x05self_\tgetTiming\0\0\0&__widl_f_update_timing_AnimationEffect\x01\0\0\x01\x0FAnimationEffect\x01\0\0\x01\x01\x05self_\x0CupdateTiming\0\0\02__widl_f_update_timing_with_timing_AnimationEffect\x01\0\0\x01\x0FAnimationEffect\x01\0\0\x01\x02\x05self_\x06timing\x0CupdateTiming\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
