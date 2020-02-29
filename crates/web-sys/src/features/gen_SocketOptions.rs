use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SocketOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SocketOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `SocketOptions`*
    pub type SocketOptions;

}

impl SocketOptions {
    ///Construct a new `SocketOptions`.
    ///
    ///*This API requires the following crate features to be activated: `SocketOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    #[cfg(feature = "TcpSocketBinaryType")]
    ///Change the `binaryType` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*

    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("binaryType"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `useSecureTransport` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SocketOptions`*

    pub fn use_secure_transport(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("useSecureTransport"),
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
