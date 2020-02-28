use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Navigator , typescript_name = Navigator ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Navigator` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub type Navigator;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = permissions ) ]
    #[cfg(feature = "Permissions")]
    #[doc = "Getter for the `permissions` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)\n\n*This API requires the following crate features to be activated: `Navigator`, `Permissions`*"]
    pub fn permissions(this: &Navigator) -> Result<Permissions, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = mimeTypes ) ]
    #[cfg(feature = "MimeTypeArray")]
    #[doc = "Getter for the `mimeTypes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`, `Navigator`*"]
    pub fn mime_types(this: &Navigator) -> Result<MimeTypeArray, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = plugins ) ]
    #[cfg(feature = "PluginArray")]
    #[doc = "Getter for the `plugins` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)\n\n*This API requires the following crate features to be activated: `Navigator`, `PluginArray`*"]
    pub fn plugins(this: &Navigator) -> Result<PluginArray, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = doNotTrack ) ]
    #[doc = "Getter for the `doNotTrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/doNotTrack)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn do_not_track(this: &Navigator) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = maxTouchPoints ) ]
    #[doc = "Getter for the `maxTouchPoints` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn max_touch_points(this: &Navigator) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = mediaCapabilities ) ]
    #[cfg(feature = "MediaCapabilities")]
    #[doc = "Getter for the `mediaCapabilities` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `Navigator`*"]
    pub fn media_capabilities(this: &Navigator) -> MediaCapabilities;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = connection ) ]
    #[cfg(feature = "NetworkInformation")]
    #[doc = "Getter for the `connection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)\n\n*This API requires the following crate features to be activated: `Navigator`, `NetworkInformation`*"]
    pub fn connection(this: &Navigator) -> Result<NetworkInformation, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = activeVRDisplays ) ]
    #[doc = "Getter for the `activeVRDisplays` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/activeVRDisplays)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn active_vr_displays(this: &Navigator) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = mediaDevices ) ]
    #[cfg(feature = "MediaDevices")]
    #[doc = "Getter for the `mediaDevices` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `Navigator`*"]
    pub fn media_devices(this: &Navigator) -> Result<MediaDevices, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = serviceWorker ) ]
    #[cfg(feature = "ServiceWorkerContainer")]
    #[doc = "Getter for the `serviceWorker` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)\n\n*This API requires the following crate features to be activated: `Navigator`, `ServiceWorkerContainer`*"]
    pub fn service_worker(this: &Navigator) -> ServiceWorkerContainer;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = presentation ) ]
    #[cfg(feature = "Presentation")]
    #[doc = "Getter for the `presentation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)\n\n*This API requires the following crate features to be activated: `Navigator`, `Presentation`*"]
    pub fn presentation(this: &Navigator) -> Result<Option<Presentation>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = credentials ) ]
    #[cfg(feature = "CredentialsContainer")]
    #[doc = "Getter for the `credentials` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`, `Navigator`*"]
    pub fn credentials(this: &Navigator) -> CredentialsContainer;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = gpu ) ]
    #[cfg(feature = "Gpu")]
    #[doc = "Getter for the `gpu` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)\n\n*This API requires the following crate features to be activated: `Gpu`, `Navigator`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn gpu(this: &Navigator) -> Gpu;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = hardwareConcurrency ) ]
    #[doc = "Getter for the `hardwareConcurrency` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn hardware_concurrency(this: &Navigator) -> f64;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = geolocation ) ]
    #[cfg(feature = "Geolocation")]
    #[doc = "Getter for the `geolocation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)\n\n*This API requires the following crate features to be activated: `Geolocation`, `Navigator`*"]
    pub fn geolocation(this: &Navigator) -> Result<Geolocation, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = appCodeName ) ]
    #[doc = "Getter for the `appCodeName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appCodeName)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_code_name(this: &Navigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = appName ) ]
    #[doc = "Getter for the `appName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appName)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_name(this: &Navigator) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = appVersion ) ]
    #[doc = "Getter for the `appVersion` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appVersion)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn app_version(this: &Navigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = platform ) ]
    #[doc = "Getter for the `platform` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn platform(this: &Navigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = userAgent ) ]
    #[doc = "Getter for the `userAgent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn user_agent(this: &Navigator) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = product ) ]
    #[doc = "Getter for the `product` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/product)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn product(this: &Navigator) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = language ) ]
    #[doc = "Getter for the `language` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn language(this: &Navigator) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = languages ) ]
    #[doc = "Getter for the `languages` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn languages(this: &Navigator) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = onLine ) ]
    #[doc = "Getter for the `onLine` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn on_line(this: &Navigator) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = storage ) ]
    #[cfg(feature = "StorageManager")]
    #[doc = "Getter for the `storage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)\n\n*This API requires the following crate features to be activated: `Navigator`, `StorageManager`*"]
    pub fn storage(this: &Navigator) -> StorageManager;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = getGamepads ) ]
    #[doc = "The `getGamepads()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn get_gamepads(this: &Navigator) -> Result<::js_sys::Array, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = getVRDisplays ) ]
    #[doc = "The `getVRDisplays()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getVRDisplays)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn get_vr_displays(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "GamepadServiceTest")]
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestGamepadServiceTest ) ]
    #[doc = "The `requestGamepadServiceTest()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestGamepadServiceTest)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`, `Navigator`*"]
    pub fn request_gamepad_service_test(this: &Navigator) -> GamepadServiceTest;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = requestMIDIAccess ) ]
    #[doc = "The `requestMIDIAccess()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn request_midi_access(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MidiOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = requestMIDIAccess ) ]
    #[doc = "The `requestMIDIAccess()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)\n\n*This API requires the following crate features to be activated: `MidiOptions`, `Navigator`*"]
    pub fn request_midi_access_with_options(
        this: &Navigator,
        options: &MidiOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestMediaKeySystemAccess ) ]
    #[doc = "The `requestMediaKeySystemAccess()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn request_media_key_system_access(
        this: &Navigator,
        key_system: &str,
        supported_configurations: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "VrServiceTest")]
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestVRServiceTest ) ]
    #[doc = "The `requestVRServiceTest()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestVRServiceTest)\n\n*This API requires the following crate features to be activated: `Navigator`, `VrServiceTest`*"]
    pub fn request_vr_service_test(this: &Navigator) -> VrServiceTest;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon(this: &Navigator, url: &str) -> Result<bool, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Blob`, `Navigator`*"]
    pub fn send_beacon_with_opt_blob(
        this: &Navigator,
        url: &str,
        data: Option<&Blob>,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_buffer_source(
        this: &Navigator,
        url: &str,
        data: Option<&::js_sys::Object>,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_u8_array(
        this: &Navigator,
        url: &str,
        data: Option<&mut [u8]>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "FormData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `FormData`, `Navigator`*"]
    pub fn send_beacon_with_opt_form_data(
        this: &Navigator,
        url: &str,
        data: Option<&FormData>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "UrlSearchParams")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`, `UrlSearchParams`*"]
    pub fn send_beacon_with_opt_url_search_params(
        this: &Navigator,
        url: &str,
        data: Option<&UrlSearchParams>,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn send_beacon_with_opt_str(
        this: &Navigator,
        url: &str,
        data: Option<&str>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "ReadableStream")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    #[doc = "The `sendBeacon()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`, `ReadableStream`*"]
    pub fn send_beacon_with_opt_readable_stream(
        this: &Navigator,
        url: &str,
        data: Option<&ReadableStream>,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = vibrate ) ]
    #[doc = "The `vibrate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn vibrate_with_duration(this: &Navigator, duration: u32) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = vibrate ) ]
    #[doc = "The `vibrate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn vibrate_with_pattern(this: &Navigator, pattern: &::wasm_bindgen::JsValue) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = registerContentHandler ) ]
    #[doc = "The `registerContentHandler()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerContentHandler)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn register_content_handler(
        this: &Navigator,
        mime_type: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = registerProtocolHandler ) ]
    #[doc = "The `registerProtocolHandler()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn register_protocol_handler(
        this: &Navigator,
        scheme: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = taintEnabled ) ]
    #[doc = "The `taintEnabled()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    pub fn taint_enabled(this: &Navigator) -> bool;
}
