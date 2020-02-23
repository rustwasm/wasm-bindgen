use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AnalyserNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AnalyserNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AnalyserNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AnalyserNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(65u32);
            inform(110u32);
            inform(97u32);
            inform(108u32);
            inform(121u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AnalyserNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for AnalyserNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AnalyserNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AnalyserNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AnalyserNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AnalyserNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AnalyserNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AnalyserNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AnalyserNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AnalyserNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AnalyserNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AnalyserNode {
        #[inline]
        fn from(obj: JsValue) -> AnalyserNode {
            AnalyserNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AnalyserNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AnalyserNode> for AnalyserNode {
        #[inline]
        fn as_ref(&self) -> &AnalyserNode {
            self
        }
    }
    impl From<AnalyserNode> for JsValue {
        #[inline]
        fn from(obj: AnalyserNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AnalyserNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AnalyserNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AnalyserNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AnalyserNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AnalyserNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AnalyserNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AnalyserNode> for AudioNode {
    #[inline]
    fn from(obj: AnalyserNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for AnalyserNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AnalyserNode> for EventTarget {
    #[inline]
    fn from(obj: AnalyserNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AnalyserNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AnalyserNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: AnalyserNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AnalyserNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AnalyserNode", feature = "BaseAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <AnalyserNode as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode", feature = "BaseAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<AnalyserNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnalyserNode", feature = "BaseAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AnalyserNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AnalyserNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_AnalyserNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AnalyserNode",
    feature = "AnalyserOptions",
    feature = "BaseAudioContext",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&AnalyserOptions as WasmDescribe>::describe();
    <AnalyserNode as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(
        feature = "AnalyserNode",
        feature = "AnalyserOptions",
        feature = "BaseAudioContext",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `AnalyserOptions`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &AnalyserOptions,
    ) -> Result<AnalyserNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AnalyserNode",
            feature = "AnalyserOptions",
            feature = "BaseAudioContext",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_AnalyserNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AnalyserOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_AnalyserNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AnalyserOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&AnalyserOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_AnalyserNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_byte_frequency_data_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `getByteFrequencyData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn get_byte_frequency_data(&self, array: &mut [u8]) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_byte_frequency_data_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_byte_frequency_data_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_byte_frequency_data_AnalyserNode(self_, array)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_byte_time_domain_data_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `getByteTimeDomainData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn get_byte_time_domain_data(&self, array: &mut [u8]) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_byte_time_domain_data_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_byte_time_domain_data_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_byte_time_domain_data_AnalyserNode(self_, array)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_float_frequency_data_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `getFloatFrequencyData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn get_float_frequency_data(&self, array: &mut [f32]) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_float_frequency_data_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_float_frequency_data_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_float_frequency_data_AnalyserNode(self_, array)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_float_time_domain_data_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `getFloatTimeDomainData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn get_float_time_domain_data(&self, array: &mut [f32]) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_float_time_domain_data_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_float_time_domain_data_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_float_time_domain_data_AnalyserNode(self_, array)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fft_size_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `fftSize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn fft_size(&self) -> u32 {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fft_size_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fft_size_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fft_size_AnalyserNode(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_fft_size_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `fftSize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn set_fft_size(&self, fft_size: u32) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_fft_size_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fft_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_fft_size_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fft_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fft_size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fft_size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fft_size);
                __widl_f_set_fft_size_AnalyserNode(self_, fft_size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frequency_bin_count_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `frequencyBinCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/frequencyBinCount)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn frequency_bin_count(&self) -> u32 {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frequency_bin_count_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frequency_bin_count_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frequency_bin_count_AnalyserNode(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_decibels_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `minDecibels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn min_decibels(&self) -> f64 {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_decibels_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_decibels_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_decibels_AnalyserNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_min_decibels_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `minDecibels` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn set_min_decibels(&self, min_decibels: f64) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_min_decibels_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                min_decibels: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_min_decibels_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            min_decibels: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(min_decibels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let min_decibels =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(min_decibels);
                __widl_f_set_min_decibels_AnalyserNode(self_, min_decibels)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_decibels_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `maxDecibels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn max_decibels(&self) -> f64 {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_decibels_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_decibels_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_decibels_AnalyserNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_decibels_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `maxDecibels` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn set_max_decibels(&self, max_decibels: f64) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_decibels_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_decibels: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_decibels_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_decibels: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max_decibels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_decibels =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_decibels);
                __widl_f_set_max_decibels_AnalyserNode(self_, max_decibels)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_smoothing_time_constant_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `smoothingTimeConstant` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn smoothing_time_constant(&self) -> f64 {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_smoothing_time_constant_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_smoothing_time_constant_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_smoothing_time_constant_AnalyserNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnalyserNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_smoothing_time_constant_AnalyserNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AnalyserNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AnalyserNode {
    #[cfg(all(feature = "AnalyserNode",))]
    #[allow(bad_style)]
    #[doc = "The `smoothingTimeConstant` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)\n\n*This API requires the following crate features to be activated: `AnalyserNode`*"]
    #[allow(clippy::all)]
    pub fn set_smoothing_time_constant(&self, smoothing_time_constant: f64) {
        #[cfg(all(feature = "AnalyserNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_smoothing_time_constant_AnalyserNode(
                self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                smoothing_time_constant: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_smoothing_time_constant_AnalyserNode(
            self_: <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            smoothing_time_constant: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(smoothing_time_constant);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnalyserNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let smoothing_time_constant =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(smoothing_time_constant);
                __widl_f_set_smoothing_time_constant_AnalyserNode(self_, smoothing_time_constant)
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
pub static __WASM_BINDGEN_GENERATED_44e57d438309a40b: [u8; 1664usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}>\x06\0\0\0\0\x10\0\0\x02\x0CAnalyserNode\x1E__widl_instanceof_AnalyserNode\0\0\0\0\x19__widl_f_new_AnalyserNode\x01\0\0\x01\x0CAnalyserNode\0\x01\x01\x07context\x03new\0\0\0&__widl_f_new_with_options_AnalyserNode\x01\0\0\x01\x0CAnalyserNode\0\x01\x02\x07context\x07options\x03new\0\0\0-__widl_f_get_byte_frequency_data_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\0\x01\x02\x05self_\x05array\x14getByteFrequencyData\0\0\0/__widl_f_get_byte_time_domain_data_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\0\x01\x02\x05self_\x05array\x15getByteTimeDomainData\0\0\0.__widl_f_get_float_frequency_data_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\0\x01\x02\x05self_\x05array\x15getFloatFrequencyData\0\0\00__widl_f_get_float_time_domain_data_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\0\x01\x02\x05self_\x05array\x16getFloatTimeDomainData\0\0\0\x1E__widl_f_fft_size_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x01\x07fftSize\x01\x01\x05self_\x07fftSize\0\0\0\"__widl_f_set_fft_size_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x02\x07fftSize\x01\x02\x05self_\x08fft_size\x07fftSize\0\0\0)__widl_f_frequency_bin_count_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x01\x11frequencyBinCount\x01\x01\x05self_\x11frequencyBinCount\0\0\0\"__widl_f_min_decibels_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x01\x0BminDecibels\x01\x01\x05self_\x0BminDecibels\0\0\0&__widl_f_set_min_decibels_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x02\x0BminDecibels\x01\x02\x05self_\x0Cmin_decibels\x0BminDecibels\0\0\0\"__widl_f_max_decibels_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x01\x0BmaxDecibels\x01\x01\x05self_\x0BmaxDecibels\0\0\0&__widl_f_set_max_decibels_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x02\x0BmaxDecibels\x01\x02\x05self_\x0Cmax_decibels\x0BmaxDecibels\0\0\0-__widl_f_smoothing_time_constant_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x01\x15smoothingTimeConstant\x01\x01\x05self_\x15smoothingTimeConstant\0\0\01__widl_f_set_smoothing_time_constant_AnalyserNode\0\0\0\x01\x0CAnalyserNode\x01\0\x02\x15smoothingTimeConstant\x01\x02\x05self_\x17smoothing_time_constant\x15smoothingTimeConstant\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
