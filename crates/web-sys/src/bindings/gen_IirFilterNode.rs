use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IIRFilterNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode)\n\n*This API requires the following crate features to be activated: `IirFilterNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IirFilterNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IirFilterNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IirFilterNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(73u32);
            inform(73u32);
            inform(82u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IirFilterNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for IirFilterNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IirFilterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IirFilterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IirFilterNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IirFilterNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IirFilterNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IirFilterNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IirFilterNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IirFilterNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IirFilterNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IirFilterNode {
        #[inline]
        fn from(obj: JsValue) -> IirFilterNode {
            IirFilterNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IirFilterNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IirFilterNode> for IirFilterNode {
        #[inline]
        fn as_ref(&self) -> &IirFilterNode {
            self
        }
    }
    impl From<IirFilterNode> for JsValue {
        #[inline]
        fn from(obj: IirFilterNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IirFilterNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IIRFilterNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IIRFilterNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IIRFilterNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IirFilterNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IirFilterNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IirFilterNode> for AudioNode {
    #[inline]
    fn from(obj: IirFilterNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for IirFilterNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IirFilterNode> for EventTarget {
    #[inline]
    fn from(obj: IirFilterNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IirFilterNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IirFilterNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: IirFilterNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IirFilterNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "IirFilterNode",
    feature = "IirFilterOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_IIRFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&IirFilterOptions as WasmDescribe>::describe();
    <IirFilterNode as WasmDescribe>::describe();
}
impl IirFilterNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "IirFilterNode",
        feature = "IirFilterOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new IIRFilterNode(..)` constructor, creating a new instance of `IIRFilterNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/IIRFilterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `IirFilterNode`, `IirFilterOptions`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &BaseAudioContext,
        options: &IirFilterOptions,
    ) -> Result<IirFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "IirFilterNode",
            feature = "IirFilterOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_IIRFilterNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&IirFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_IIRFilterNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&IirFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&IirFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_IIRFilterNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IirFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_frequency_response_IIRFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&IirFilterNode as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IirFilterNode {
    #[cfg(all(feature = "IirFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `getFrequencyResponse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)\n\n*This API requires the following crate features to be activated: `IirFilterNode`*"]
    #[allow(clippy::all)]
    pub fn get_frequency_response(
        &self,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    ) {
        #[cfg(all(feature = "IirFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_frequency_response_IIRFilterNode(
                self_: <&IirFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frequency_hz: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mag_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                phase_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_frequency_response_IIRFilterNode(
            self_: <&IirFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frequency_hz: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mag_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            phase_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(frequency_hz);
            drop(mag_response);
            drop(phase_response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IirFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frequency_hz =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frequency_hz);
                let mag_response =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mag_response);
                let phase_response =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(phase_response);
                __widl_f_get_frequency_response_IIRFilterNode(
                    self_,
                    frequency_hz,
                    mag_response,
                    phase_response,
                )
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
pub static __WASM_BINDGEN_GENERATED_9163f63fac31d94e: [u8; 366usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"},\x01\0\0\0\0\x03\0\0\x02\rIIRFilterNode\x1F__widl_instanceof_IIRFilterNode\0\0\0\0\x1A__widl_f_new_IIRFilterNode\x01\0\0\x01\rIIRFilterNode\0\x01\x02\x07context\x07options\x03new\0\0\0-__widl_f_get_frequency_response_IIRFilterNode\0\0\0\x01\rIIRFilterNode\x01\0\0\x01\x04\x05self_\x0Cfrequency_hz\x0Cmag_response\x0Ephase_response\x14getFrequencyResponse\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
