use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OfflineAudioContext` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OfflineAudioContext {
    obj: BaseAudioContext,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OfflineAudioContext: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OfflineAudioContext {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(79u32);
            inform(102u32);
            inform(102u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for OfflineAudioContext {
        type Target = BaseAudioContext;
        #[inline]
        fn deref(&self) -> &BaseAudioContext {
            &self.obj
        }
    }
    impl IntoWasmAbi for OfflineAudioContext {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OfflineAudioContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OfflineAudioContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OfflineAudioContext {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OfflineAudioContext {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OfflineAudioContext {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OfflineAudioContext {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OfflineAudioContext {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OfflineAudioContext>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OfflineAudioContext {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OfflineAudioContext {
        #[inline]
        fn from(obj: JsValue) -> OfflineAudioContext {
            OfflineAudioContext { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OfflineAudioContext {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OfflineAudioContext> for OfflineAudioContext {
        #[inline]
        fn as_ref(&self) -> &OfflineAudioContext {
            self
        }
    }
    impl From<OfflineAudioContext> for JsValue {
        #[inline]
        fn from(obj: OfflineAudioContext) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OfflineAudioContext {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OfflineAudioContext(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OfflineAudioContext(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OfflineAudioContext(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OfflineAudioContext { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OfflineAudioContext) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OfflineAudioContext> for BaseAudioContext {
    #[inline]
    fn from(obj: OfflineAudioContext) -> BaseAudioContext {
        use wasm_bindgen::JsCast;
        BaseAudioContext::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<BaseAudioContext> for OfflineAudioContext {
    #[inline]
    fn as_ref(&self) -> &BaseAudioContext {
        use wasm_bindgen::JsCast;
        BaseAudioContext::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OfflineAudioContext> for EventTarget {
    #[inline]
    fn from(obj: OfflineAudioContext) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for OfflineAudioContext {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OfflineAudioContext> for ::js_sys::Object {
    #[inline]
    fn from(obj: OfflineAudioContext) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OfflineAudioContext {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "OfflineAudioContext",
    feature = "OfflineAudioContextOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_context_options_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContextOptions as WasmDescribe>::describe();
    <OfflineAudioContext as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(
        feature = "OfflineAudioContext",
        feature = "OfflineAudioContextOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `OfflineAudioContextOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_context_options(
        context_options: &OfflineAudioContextOptions,
    ) -> Result<OfflineAudioContext, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "OfflineAudioContext",
            feature = "OfflineAudioContextOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_context_options_OfflineAudioContext(
                context_options : < & OfflineAudioContextOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_context_options_OfflineAudioContext(
            context_options : < & OfflineAudioContextOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context_options =
                    <&OfflineAudioContextOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        context_options,
                    );
                __widl_f_new_with_context_options_OfflineAudioContext(context_options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_number_of_channels_and_length_and_sample_rate_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <OfflineAudioContext as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new_with_number_of_channels_and_length_and_sample_rate(
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<OfflineAudioContext, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_number_of_channels_and_length_and_sample_rate_OfflineAudioContext(
                number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_number_of_channels_and_length_and_sample_rate_OfflineAudioContext(
            number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(number_of_channels);
            drop(length);
            drop(sample_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let number_of_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_channels);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                let sample_rate =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sample_rate);
                __widl_f_new_with_number_of_channels_and_length_and_sample_rate_OfflineAudioContext(
                    number_of_channels,
                    length,
                    sample_rate,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OfflineAudioContext as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_rendering_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `startRendering()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/startRendering)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn start_rendering(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_rendering_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_rendering_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_rendering_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/length)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_OfflineAudioContext(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncomplete_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn oncomplete(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncomplete_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncomplete_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncomplete_OfflineAudioContext(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncomplete_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn set_oncomplete(&self, oncomplete: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncomplete_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncomplete : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncomplete_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncomplete: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncomplete);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncomplete =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncomplete,
                    );
                __widl_f_set_oncomplete_OfflineAudioContext(self_, oncomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_analyser_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AnalyserNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AnalyserNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createAnalyser()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createAnalyser)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_analyser(&self) -> Result<AnalyserNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnalyserNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_analyser_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_analyser_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_analyser_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BiquadFilterNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_biquad_filter_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <BiquadFilterNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "BiquadFilterNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBiquadFilter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBiquadFilter)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_biquad_filter(&self) -> Result<BiquadFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BiquadFilterNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_biquad_filter_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_biquad_filter_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_biquad_filter_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_buffer_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <AudioBuffer as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioBuffer", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_buffer(
        &self,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_buffer_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_buffer_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_channels);
            drop(length);
            drop(sample_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_channels);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                let sample_rate =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sample_rate);
                __widl_f_create_buffer_OfflineAudioContext(
                    self_,
                    number_of_channels,
                    length,
                    sample_rate,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_buffer_source_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AudioBufferSourceNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioBufferSourceNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBufferSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBufferSource)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_buffer_source(&self) -> Result<AudioBufferSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_buffer_source_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_buffer_source_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_buffer_source_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_merger_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <ChannelMergerNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelMerger()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_channel_merger(&self) -> Result<ChannelMergerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_merger_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_merger_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_channel_merger_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_merger_with_number_of_inputs_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ChannelMergerNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelMerger()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_channel_merger_with_number_of_inputs(
        &self,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ChannelMergerNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_merger_with_number_of_inputs_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_inputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_merger_with_number_of_inputs_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_inputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_inputs);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_inputs =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_inputs);
                __widl_f_create_channel_merger_with_number_of_inputs_OfflineAudioContext(
                    self_,
                    number_of_inputs,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_splitter_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelSplitter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_channel_splitter(&self) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_splitter_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_splitter_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_channel_splitter_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_splitter_with_number_of_outputs_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelSplitter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_channel_splitter_with_number_of_outputs(
        &self,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ChannelSplitterNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_splitter_with_number_of_outputs_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_outputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_splitter_with_number_of_outputs_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_outputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_outputs);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_outputs =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_outputs);
                __widl_f_create_channel_splitter_with_number_of_outputs_OfflineAudioContext(
                    self_,
                    number_of_outputs,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_constant_source_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <ConstantSourceNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ConstantSourceNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createConstantSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConstantSource)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_constant_source(&self) -> Result<ConstantSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConstantSourceNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_constant_source_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_constant_source_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_constant_source_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ConvolverNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_convolver_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <ConvolverNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "ConvolverNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createConvolver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConvolver)\n\n*This API requires the following crate features to be activated: `ConvolverNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_convolver(&self) -> Result<ConvolverNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConvolverNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_convolver_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_convolver_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_convolver_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_delay_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <DelayNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createDelay()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_delay(&self) -> Result<DelayNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_delay_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_delay_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_delay_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DelayNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_delay_with_max_delay_time_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DelayNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createDelay()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_delay_with_max_delay_time(
        &self,
        max_delay_time: f64,
    ) -> Result<DelayNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DelayNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_delay_with_max_delay_time_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_delay_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_delay_with_max_delay_time_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_delay_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(max_delay_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_delay_time =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_delay_time);
                __widl_f_create_delay_with_max_delay_time_OfflineAudioContext(self_, max_delay_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DelayNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "DynamicsCompressorNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_dynamics_compressor_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <DynamicsCompressorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "DynamicsCompressorNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createDynamicsCompressor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDynamicsCompressor)\n\n*This API requires the following crate features to be activated: `DynamicsCompressorNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_dynamics_compressor(
        &self,
    ) -> Result<DynamicsCompressorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DynamicsCompressorNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_dynamics_compressor_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_dynamics_compressor_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_dynamics_compressor_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GainNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_gain_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <GainNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "GainNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createGain()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createGain)\n\n*This API requires the following crate features to be activated: `GainNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_gain(&self) -> Result<GainNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GainNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_gain_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_gain_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_gain_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GainNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "IirFilterNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_iir_filter_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IirFilterNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "IirFilterNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createIIRFilter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createIIRFilter)\n\n*This API requires the following crate features to be activated: `IirFilterNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_iir_filter(
        &self,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IirFilterNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_iir_filter_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                feedforward: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                feedback: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_iir_filter_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            feedforward: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            feedback: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(feedforward);
            drop(feedback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let feedforward =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        feedforward,
                    );
                let feedback =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        feedback,
                    );
                __widl_f_create_iir_filter_OfflineAudioContext(self_, feedforward, feedback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_oscillator_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <OscillatorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createOscillator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createOscillator)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn create_oscillator(&self) -> Result<OscillatorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_oscillator_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_oscillator_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_oscillator_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_panner_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <PannerNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createPanner()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPanner)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn create_panner(&self) -> Result<PannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_panner_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_panner_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_panner_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "PeriodicWave",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_periodic_wave_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "PeriodicWave",))]
    #[allow(bad_style)]
    #[doc = "The `createPeriodicWave()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
    #[allow(clippy::all)]
    pub fn create_periodic_wave(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "PeriodicWave",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_periodic_wave_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_periodic_wave_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(real);
            drop(imag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let real = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(real);
                let imag = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imag);
                __widl_f_create_periodic_wave_OfflineAudioContext(self_, real, imag)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "OfflineAudioContext",
    feature = "PeriodicWave",
    feature = "PeriodicWaveConstraints",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_periodic_wave_with_constraints_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&PeriodicWaveConstraints as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(
        feature = "OfflineAudioContext",
        feature = "PeriodicWave",
        feature = "PeriodicWaveConstraints",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPeriodicWave()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    #[allow(clippy::all)]
    pub fn create_periodic_wave_with_constraints(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "OfflineAudioContext",
            feature = "PeriodicWave",
            feature = "PeriodicWaveConstraints",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_periodic_wave_with_constraints_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                constraints: <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_periodic_wave_with_constraints_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            constraints: <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(real);
            drop(imag);
            drop(constraints);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let real = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(real);
                let imag = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imag);
                let constraints =
                    <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        constraints,
                    );
                __widl_f_create_periodic_wave_with_constraints_OfflineAudioContext(
                    self_,
                    real,
                    imag,
                    constraints,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor(&self) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_script_processor_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size(
        &self,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                __widl_f_create_script_processor_with_buffer_size_OfflineAudioContext(
                    self_,
                    buffer_size,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            drop(number_of_input_channels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                let number_of_input_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_input_channels);
                __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_OfflineAudioContext ( self_ , buffer_size , number_of_input_channels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_output_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_output_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            drop(number_of_input_channels);
            drop(number_of_output_channels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                let number_of_input_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_input_channels);
                let number_of_output_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        number_of_output_channels,
                    );
                __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_OfflineAudioContext ( self_ , buffer_size , number_of_input_channels , number_of_output_channels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "StereoPannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_stereo_panner_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <StereoPannerNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "StereoPannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createStereoPanner()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createStereoPanner)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `StereoPannerNode`*"]
    #[allow(clippy::all)]
    pub fn create_stereo_panner(&self) -> Result<StereoPannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "StereoPannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_stereo_panner_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_stereo_panner_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_stereo_panner_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext", feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_wave_shaper_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <WaveShaperNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext", feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `createWaveShaper()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createWaveShaper)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`, `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn create_wave_shaper(&self) -> Result<WaveShaperNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext", feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_wave_shaper_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_wave_shaper_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_wave_shaper_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                __widl_f_decode_audio_data_OfflineAudioContext(self_, audio_data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_with_success_callback_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data_with_success_callback(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_with_success_callback_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_with_success_callback_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_decode_audio_data_with_success_callback_OfflineAudioContext(
                    self_,
                    audio_data,
                    success_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_with_success_callback_and_error_callback_OfflineAudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_with_success_callback_and_error_callback_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_with_success_callback_and_error_callback_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_decode_audio_data_with_success_callback_and_error_callback_OfflineAudioContext ( self_ , audio_data , success_callback , error_callback )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_resume_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `resume()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/resume)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn resume(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_resume_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_resume_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_resume_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioDestinationNode", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_destination_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AudioDestinationNode as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioDestinationNode", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `destination` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/destination)\n\n*This API requires the following crate features to be activated: `AudioDestinationNode`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn destination(&self) -> AudioDestinationNode {
        #[cfg(all(feature = "AudioDestinationNode", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_destination_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_destination_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_destination_OfflineAudioContext(self_)
            };
            <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_rate_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `sampleRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/sampleRate)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn sample_rate(&self) -> f32 {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_rate_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_rate_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sample_rate_OfflineAudioContext(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/currentTime)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> f64 {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_OfflineAudioContext(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioListener", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_listener_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AudioListener as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioListener", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `listener` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/listener)\n\n*This API requires the following crate features to be activated: `AudioListener`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn listener(&self) -> AudioListener {
        #[cfg(all(feature = "AudioListener", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_listener_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioListener as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_listener_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioListener as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_listener_OfflineAudioContext(self_)
            };
            <AudioListener as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContextState", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AudioContextState as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioContextState", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/state)\n\n*This API requires the following crate features to be activated: `AudioContextState`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> AudioContextState {
        #[cfg(all(feature = "AudioContextState", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_OfflineAudioContext(self_)
            };
            <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioWorklet", feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_audio_worklet_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <AudioWorklet as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "AudioWorklet", feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `audioWorklet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/audioWorklet)\n\n*This API requires the following crate features to be activated: `AudioWorklet`, `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn audio_worklet(&self) -> Result<AudioWorklet, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioWorklet", feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_audio_worklet_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_audio_worklet_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_audio_worklet_OfflineAudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_OfflineAudioContext(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_OfflineAudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineAudioContext as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineAudioContext {
    #[cfg(all(feature = "OfflineAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_OfflineAudioContext(
                self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_OfflineAudioContext(
            self_: <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstatechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_OfflineAudioContext(self_, onstatechange)
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
pub static __WASM_BINDGEN_GENERATED_ffbf1b41074bf572: [u8; 5296usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x14\0\0\0\0,\0\0\x02\x13OfflineAudioContext%__widl_instanceof_OfflineAudioContext\x01\x06webkit\0\0\05__widl_f_new_with_context_options_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\0\x01\x01\x0Fcontext_options\x03new\0\0\0S__widl_f_new_with_number_of_channels_and_length_and_sample_rate_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\0\x01\x03\x12number_of_channels\x06length\x0Bsample_rate\x03new\0\0\0,__widl_f_start_rendering_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x0EstartRendering\0\0\0#__widl_f_length_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0'__widl_f_oncomplete_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\noncomplete\x01\x01\x05self_\noncomplete\0\0\0+__widl_f_set_oncomplete_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x02\noncomplete\x01\x02\x05self_\noncomplete\noncomplete\0\0\0,__widl_f_create_analyser_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x0EcreateAnalyser\0\0\01__widl_f_create_biquad_filter_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x12createBiquadFilter\0\0\0*__widl_f_create_buffer_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x04\x05self_\x12number_of_channels\x06length\x0Bsample_rate\x0CcreateBuffer\0\0\01__widl_f_create_buffer_source_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x12createBufferSource\0\0\02__widl_f_create_channel_merger_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x13createChannelMerger\0\0\0H__widl_f_create_channel_merger_with_number_of_inputs_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x02\x05self_\x10number_of_inputs\x13createChannelMerger\0\0\04__widl_f_create_channel_splitter_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x15createChannelSplitter\0\0\0K__widl_f_create_channel_splitter_with_number_of_outputs_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x02\x05self_\x11number_of_outputs\x15createChannelSplitter\0\0\03__widl_f_create_constant_source_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x14createConstantSource\0\0\0-__widl_f_create_convolver_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x0FcreateConvolver\0\0\0)__widl_f_create_delay_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x0BcreateDelay\0\0\0=__widl_f_create_delay_with_max_delay_time_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x02\x05self_\x0Emax_delay_time\x0BcreateDelay\0\0\07__widl_f_create_dynamics_compressor_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x18createDynamicsCompressor\0\0\0(__widl_f_create_gain_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\ncreateGain\0\0\0.__widl_f_create_iir_filter_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x03\x05self_\x0Bfeedforward\x08feedback\x0FcreateIIRFilter\0\0\0.__widl_f_create_oscillator_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x10createOscillator\0\0\0*__widl_f_create_panner_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x0CcreatePanner\0\0\01__widl_f_create_periodic_wave_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x03\x05self_\x04real\x04imag\x12createPeriodicWave\0\0\0B__widl_f_create_periodic_wave_with_constraints_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x04\x05self_\x04real\x04imag\x0Bconstraints\x12createPeriodicWave\0\0\04__widl_f_create_script_processor_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x15createScriptProcessor\0\0\0E__widl_f_create_script_processor_with_buffer_size_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x02\x05self_\x0Bbuffer_size\x15createScriptProcessor\0\0\0b__widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x03\x05self_\x0Bbuffer_size\x18number_of_input_channels\x15createScriptProcessor\0\0\0\x80\x01__widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x04\x05self_\x0Bbuffer_size\x18number_of_input_channels\x19number_of_output_channels\x15createScriptProcessor\0\0\01__widl_f_create_stereo_panner_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x12createStereoPanner\0\0\0/__widl_f_create_wave_shaper_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x10createWaveShaper\0\0\0.__widl_f_decode_audio_data_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x02\x05self_\naudio_data\x0FdecodeAudioData\0\0\0D__widl_f_decode_audio_data_with_success_callback_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x03\x05self_\naudio_data\x10success_callback\x0FdecodeAudioData\0\0\0W__widl_f_decode_audio_data_with_success_callback_and_error_callback_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x04\x05self_\naudio_data\x10success_callback\x0Eerror_callback\x0FdecodeAudioData\0\0\0#__widl_f_resume_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\0\x01\x01\x05self_\x06resume\0\0\0(__widl_f_destination_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\x0Bdestination\x01\x01\x05self_\x0Bdestination\0\0\0(__widl_f_sample_rate_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\nsampleRate\x01\x01\x05self_\nsampleRate\0\0\0)__widl_f_current_time_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0%__widl_f_listener_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\x08listener\x01\x01\x05self_\x08listener\0\0\0\"__widl_f_state_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0*__widl_f_audio_worklet_OfflineAudioContext\x01\0\0\x01\x13OfflineAudioContext\x01\0\x01\x0CaudioWorklet\x01\x01\x05self_\x0CaudioWorklet\0\0\0*__widl_f_onstatechange_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0.__widl_f_set_onstatechange_OfflineAudioContext\0\0\0\x01\x13OfflineAudioContext\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
