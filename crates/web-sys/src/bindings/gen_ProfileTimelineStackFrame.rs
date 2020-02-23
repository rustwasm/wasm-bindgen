use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ProfileTimelineStackFrame` dictionary\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
pub struct ProfileTimelineStackFrame {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ProfileTimelineStackFrame {
    #[doc = "Construct a new `ProfileTimelineStackFrame`\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn new() -> ProfileTimelineStackFrame {
        let mut _ret = ProfileTimelineStackFrame {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `asyncCause` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn async_cause(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("asyncCause"),
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
    #[doc = "Configure the `asyncParent` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn async_parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("asyncParent"),
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
    #[doc = "Configure the `column` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn column(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("column"),
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
    #[doc = "Configure the `functionDisplayName` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn function_display_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("functionDisplayName"),
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
    #[doc = "Configure the `line` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn line(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("line"),
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
    #[doc = "Configure the `parent` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("parent"),
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
    #[doc = "Configure the `source` field of this object\n\n\n*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn source(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("source"),
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
const _CONST_ProfileTimelineStackFrame: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ProfileTimelineStackFrame> for JsValue {
        #[inline]
        fn from(val: ProfileTimelineStackFrame) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ProfileTimelineStackFrame {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ProfileTimelineStackFrame {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ProfileTimelineStackFrame {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ProfileTimelineStackFrame {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ProfileTimelineStackFrame {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ProfileTimelineStackFrame {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ProfileTimelineStackFrame {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ProfileTimelineStackFrame {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ProfileTimelineStackFrame {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ProfileTimelineStackFrame {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ProfileTimelineStackFrame>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ProfileTimelineStackFrame {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ProfileTimelineStackFrame {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ProfileTimelineStackFrame {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ProfileTimelineStackFrame) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c2c92225c5cbd87e: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
