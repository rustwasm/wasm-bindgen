use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ClientQueryOptions` dictionary\n\n\n*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
pub struct ClientQueryOptions {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ClientQueryOptions {
    #[doc = "Construct a new `ClientQueryOptions`\n\n\n*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    pub fn new() -> ClientQueryOptions {
        let mut _ret = ClientQueryOptions {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `includeUncontrolled` field of this object\n\n\n*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub fn include_uncontrolled(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("includeUncontrolled"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "ClientType",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `type` field of this object\n\n\n*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    pub fn type_(&mut self, val: ClientType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("type"),
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
const _CONST_ClientQueryOptions: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ClientQueryOptions> for JsValue {
        #[inline]
        fn from(val: ClientQueryOptions) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ClientQueryOptions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ClientQueryOptions {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ClientQueryOptions {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ClientQueryOptions {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ClientQueryOptions {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ClientQueryOptions {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ClientQueryOptions {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ClientQueryOptions {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ClientQueryOptions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ClientQueryOptions {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ClientQueryOptions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ClientQueryOptions {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ClientQueryOptions {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ClientQueryOptions {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ClientQueryOptions) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ab40fcb9a461cf82: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
