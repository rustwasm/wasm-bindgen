use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WaveShaperNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode)\n\n*This API requires the following crate features to be activated: `WaveShaperNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WaveShaperNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WaveShaperNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WaveShaperNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(87u32);
            inform(97u32);
            inform(118u32);
            inform(101u32);
            inform(83u32);
            inform(104u32);
            inform(97u32);
            inform(112u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for WaveShaperNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for WaveShaperNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WaveShaperNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WaveShaperNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WaveShaperNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WaveShaperNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WaveShaperNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WaveShaperNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WaveShaperNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WaveShaperNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WaveShaperNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WaveShaperNode {
        #[inline]
        fn from(obj: JsValue) -> WaveShaperNode {
            WaveShaperNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WaveShaperNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WaveShaperNode> for WaveShaperNode {
        #[inline]
        fn as_ref(&self) -> &WaveShaperNode {
            self
        }
    }
    impl From<WaveShaperNode> for JsValue {
        #[inline]
        fn from(obj: WaveShaperNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WaveShaperNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WaveShaperNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WaveShaperNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WaveShaperNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WaveShaperNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WaveShaperNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WaveShaperNode> for AudioNode {
    #[inline]
    fn from(obj: WaveShaperNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for WaveShaperNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WaveShaperNode> for EventTarget {
    #[inline]
    fn from(obj: WaveShaperNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for WaveShaperNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WaveShaperNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: WaveShaperNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WaveShaperNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <WaveShaperNode as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<WaveShaperNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_WaveShaperNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_WaveShaperNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                __widl_f_new_WaveShaperNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "WaveShaperNode",
    feature = "WaveShaperOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&WaveShaperOptions as WasmDescribe>::describe();
    <WaveShaperNode as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "WaveShaperNode",
        feature = "WaveShaperOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`, `WaveShaperOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &WaveShaperOptions,
    ) -> Result<WaveShaperNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "WaveShaperNode",
            feature = "WaveShaperOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_WaveShaperNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&WaveShaperOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_WaveShaperNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&WaveShaperOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let options =
                    <&WaveShaperOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_WaveShaperNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_curve_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WaveShaperNode as WasmDescribe>::describe();
    <Option<Vec<f32>> as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `curve` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)\n\n*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn curve(&self) -> Option<Vec<f32>> {
        #[cfg(all(feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_curve_WaveShaperNode(
                self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_curve_WaveShaperNode(
            self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_curve_WaveShaperNode(self_)
            };
            <Option<Vec<f32>> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_curve_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WaveShaperNode as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `curve` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)\n\n*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn set_curve(&self, curve: Option<&mut [f32]>) {
        #[cfg(all(feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_curve_WaveShaperNode(
                self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                curve: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_curve_WaveShaperNode(
            self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            curve: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(curve);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let curve =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(curve);
                __widl_f_set_curve_WaveShaperNode(self_, curve)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oversample_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WaveShaperNode as WasmDescribe>::describe();
    <OverSampleType as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `oversample` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)\n\n*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn oversample(&self) -> OverSampleType {
        #[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oversample_WaveShaperNode(
                self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OverSampleType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oversample_WaveShaperNode(
            self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OverSampleType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oversample_WaveShaperNode(self_)
            };
            <OverSampleType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oversample_WaveShaperNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WaveShaperNode as WasmDescribe>::describe();
    <OverSampleType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WaveShaperNode {
    #[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `oversample` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)\n\n*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn set_oversample(&self, oversample: OverSampleType) {
        #[cfg(all(feature = "OverSampleType", feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oversample_WaveShaperNode(
                self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oversample: <OverSampleType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oversample_WaveShaperNode(
            self_: <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oversample: <OverSampleType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oversample);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WaveShaperNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oversample =
                    <OverSampleType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(oversample);
                __widl_f_set_oversample_WaveShaperNode(self_, oversample)
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
pub static __WASM_BINDGEN_GENERATED_6758a26caa99338a: [u8; 663usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}U\x02\0\0\0\0\x07\0\0\x02\x0EWaveShaperNode __widl_instanceof_WaveShaperNode\0\0\0\0\x1B__widl_f_new_WaveShaperNode\x01\0\0\x01\x0EWaveShaperNode\0\x01\x01\x07context\x03new\0\0\0(__widl_f_new_with_options_WaveShaperNode\x01\0\0\x01\x0EWaveShaperNode\0\x01\x02\x07context\x07options\x03new\0\0\0\x1D__widl_f_curve_WaveShaperNode\0\0\0\x01\x0EWaveShaperNode\x01\0\x01\x05curve\x01\x01\x05self_\x05curve\0\0\0!__widl_f_set_curve_WaveShaperNode\0\0\0\x01\x0EWaveShaperNode\x01\0\x02\x05curve\x01\x02\x05self_\x05curve\x05curve\0\0\0\"__widl_f_oversample_WaveShaperNode\0\0\0\x01\x0EWaveShaperNode\x01\0\x01\noversample\x01\x01\x05self_\noversample\0\0\0&__widl_f_set_oversample_WaveShaperNode\0\0\0\x01\x0EWaveShaperNode\x01\0\x02\noversample\x01\x02\x05self_\noversample\noversample\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
