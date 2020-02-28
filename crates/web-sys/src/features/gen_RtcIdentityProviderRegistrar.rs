use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = RTCIdentityProviderRegistrar , typescript_name = RTCIdentityProviderRegistrar ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderRegistrar` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar)\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderRegistrar`*"]
    pub type RtcIdentityProviderRegistrar;
    #[cfg(feature = "RtcIdentityProvider")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCIdentityProviderRegistrar" , js_name = register ) ]
    #[doc = "The `register()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar/register)\n\n*This API requires the following crate features to be activated: `RtcIdentityProvider`, `RtcIdentityProviderRegistrar`*"]
    pub fn register(this: &RtcIdentityProviderRegistrar, idp: &RtcIdentityProvider);
}
