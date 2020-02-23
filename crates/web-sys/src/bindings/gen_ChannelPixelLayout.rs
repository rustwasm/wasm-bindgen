use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ChannelPixelLayout` dictionary\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
pub struct ChannelPixelLayout {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ChannelPixelLayout {
    #[doc = "Construct a new `ChannelPixelLayout`\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`, `ChannelPixelLayoutDataType`*"]
    pub fn new(
        data_type: ChannelPixelLayoutDataType,
        height: u32,
        offset: u32,
        skip: u32,
        stride: u32,
        width: u32,
    ) -> ChannelPixelLayout {
        let mut _ret = ChannelPixelLayout {
            obj: ::js_sys::Object::new(),
        };
        _ret.data_type(data_type);
        _ret.height(height);
        _ret.offset(offset);
        _ret.skip(skip);
        _ret.stride(stride);
        _ret.width(width);
        return _ret;
    }
    #[cfg(all(feature = "ChannelPixelLayoutDataType",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `dataType` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`, `ChannelPixelLayoutDataType`*"]
    pub fn data_type(&mut self, val: ChannelPixelLayoutDataType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("dataType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `height` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
    pub fn height(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("height"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `offset` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
    pub fn offset(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("offset"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `skip` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
    pub fn skip(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("skip"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `stride` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
    pub fn stride(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("stride"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `width` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelPixelLayout`*"]
    pub fn width(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("width"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_ChannelPixelLayout: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ChannelPixelLayout> for JsValue {
        #[inline]
        fn from(val: ChannelPixelLayout) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ChannelPixelLayout {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ChannelPixelLayout {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ChannelPixelLayout {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ChannelPixelLayout {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ChannelPixelLayout {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ChannelPixelLayout {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ChannelPixelLayout {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ChannelPixelLayout {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ChannelPixelLayout {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ChannelPixelLayout {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ChannelPixelLayout>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ChannelPixelLayout {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ChannelPixelLayout {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ChannelPixelLayout {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ChannelPixelLayout) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d274cb8445df5da2: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
