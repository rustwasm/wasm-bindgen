use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Navigator , typescript_type = "Navigator" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Navigator` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub type Navigator;

    #[cfg(feature = "Permissions")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = permissions ) ]
    ///Getter for the `permissions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `Permissions`*
    pub fn permissions(this: &Navigator) -> Result<Permissions, JsValue>;

    #[cfg(feature = "MimeTypeArray")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = mimeTypes ) ]
    ///Getter for the `mimeTypes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)
    ///
    ///*This API requires the following crate features to be activated: `MimeTypeArray`, `Navigator`*
    pub fn mime_types(this: &Navigator) -> Result<MimeTypeArray, JsValue>;

    #[cfg(feature = "PluginArray")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = plugins ) ]
    ///Getter for the `plugins` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `PluginArray`*
    pub fn plugins(this: &Navigator) -> Result<PluginArray, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = doNotTrack ) ]
    ///Getter for the `doNotTrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/doNotTrack)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn do_not_track(this: &Navigator) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = maxTouchPoints ) ]
    ///Getter for the `maxTouchPoints` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn max_touch_points(this: &Navigator) -> i32;

    #[cfg(feature = "MediaCapabilities")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = mediaCapabilities ) ]
    ///Getter for the `mediaCapabilities` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilities`, `Navigator`*
    pub fn media_capabilities(this: &Navigator) -> MediaCapabilities;

    #[cfg(feature = "NetworkInformation")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = connection ) ]
    ///Getter for the `connection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `NetworkInformation`*
    pub fn connection(this: &Navigator) -> Result<NetworkInformation, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = activeVRDisplays ) ]
    ///Getter for the `activeVRDisplays` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/activeVRDisplays)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn active_vr_displays(this: &Navigator) -> ::js_sys::Array;

    #[cfg(feature = "MediaDevices")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = mediaDevices ) ]
    ///Getter for the `mediaDevices` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`, `Navigator`*
    pub fn media_devices(this: &Navigator) -> Result<MediaDevices, JsValue>;

    #[cfg(feature = "ServiceWorkerContainer")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = serviceWorker ) ]
    ///Getter for the `serviceWorker` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `ServiceWorkerContainer`*
    pub fn service_worker(this: &Navigator) -> ServiceWorkerContainer;

    #[cfg(feature = "Presentation")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = presentation ) ]
    ///Getter for the `presentation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `Presentation`*
    pub fn presentation(this: &Navigator) -> Result<Option<Presentation>, JsValue>;

    #[cfg(feature = "CredentialsContainer")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = credentials ) ]
    ///Getter for the `credentials` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)
    ///
    ///*This API requires the following crate features to be activated: `CredentialsContainer`, `Navigator`*
    pub fn credentials(this: &Navigator) -> CredentialsContainer;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = gpu ) ]
    ///Getter for the `gpu` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)
    ///
    ///*This API requires the following crate features to be activated: `Gpu`, `Navigator`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn gpu(this: &Navigator) -> Gpu;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = hardwareConcurrency ) ]
    ///Getter for the `hardwareConcurrency` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn hardware_concurrency(this: &Navigator) -> f64;

    #[cfg(feature = "Geolocation")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = geolocation ) ]
    ///Getter for the `geolocation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)
    ///
    ///*This API requires the following crate features to be activated: `Geolocation`, `Navigator`*
    pub fn geolocation(this: &Navigator) -> Result<Geolocation, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = appCodeName ) ]
    ///Getter for the `appCodeName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appCodeName)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn app_code_name(this: &Navigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = appName ) ]
    ///Getter for the `appName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appName)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn app_name(this: &Navigator) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = appVersion ) ]
    ///Getter for the `appVersion` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appVersion)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn app_version(this: &Navigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = platform ) ]
    ///Getter for the `platform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn platform(this: &Navigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Navigator" , js_name = userAgent ) ]
    ///Getter for the `userAgent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn user_agent(this: &Navigator) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = product ) ]
    ///Getter for the `product` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/product)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn product(this: &Navigator) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = language ) ]
    ///Getter for the `language` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn language(this: &Navigator) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = languages ) ]
    ///Getter for the `languages` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn languages(this: &Navigator) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = onLine ) ]
    ///Getter for the `onLine` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn on_line(this: &Navigator) -> bool;

    #[cfg(feature = "StorageManager")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Navigator" , js_name = storage ) ]
    ///Getter for the `storage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `StorageManager`*
    pub fn storage(this: &Navigator) -> StorageManager;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = getGamepads ) ]
    ///The `getGamepads()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn get_gamepads(this: &Navigator) -> Result<::js_sys::Array, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = getVRDisplays ) ]
    ///The `getVRDisplays()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getVRDisplays)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn get_vr_displays(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "GamepadServiceTest")]
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestGamepadServiceTest ) ]
    ///The `requestGamepadServiceTest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestGamepadServiceTest)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`, `Navigator`*
    pub fn request_gamepad_service_test(this: &Navigator) -> GamepadServiceTest;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = requestMIDIAccess ) ]
    ///The `requestMIDIAccess()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn request_midi_access(this: &Navigator) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "MidiOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = requestMIDIAccess ) ]
    ///The `requestMIDIAccess()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    ///
    ///*This API requires the following crate features to be activated: `MidiOptions`, `Navigator`*
    pub fn request_midi_access_with_options(
        this: &Navigator,
        options: &MidiOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestMediaKeySystemAccess ) ]
    ///The `requestMediaKeySystemAccess()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn request_media_key_system_access(
        this: &Navigator,
        key_system: &str,
        supported_configurations: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;

    #[cfg(feature = "VrServiceTest")]
    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = requestVRServiceTest ) ]
    ///The `requestVRServiceTest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestVRServiceTest)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `VrServiceTest`*
    pub fn request_vr_service_test(this: &Navigator) -> VrServiceTest;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn send_beacon(this: &Navigator, url: &str) -> Result<bool, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Navigator`*
    pub fn send_beacon_with_opt_blob(
        this: &Navigator,
        url: &str,
        data: Option<&Blob>,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn send_beacon_with_opt_buffer_source(
        this: &Navigator,
        url: &str,
        data: Option<&::js_sys::Object>,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn send_beacon_with_opt_u8_array(
        this: &Navigator,
        url: &str,
        data: Option<&mut [u8]>,
    ) -> Result<bool, JsValue>;

    #[cfg(feature = "FormData")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `FormData`, `Navigator`*
    pub fn send_beacon_with_opt_form_data(
        this: &Navigator,
        url: &str,
        data: Option<&FormData>,
    ) -> Result<bool, JsValue>;

    #[cfg(feature = "UrlSearchParams")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `UrlSearchParams`*
    pub fn send_beacon_with_opt_url_search_params(
        this: &Navigator,
        url: &str,
        data: Option<&UrlSearchParams>,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn send_beacon_with_opt_str(
        this: &Navigator,
        url: &str,
        data: Option<&str>,
    ) -> Result<bool, JsValue>;

    #[cfg(feature = "ReadableStream")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = sendBeacon ) ]
    ///The `sendBeacon()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`, `ReadableStream`*
    pub fn send_beacon_with_opt_readable_stream(
        this: &Navigator,
        url: &str,
        data: Option<&ReadableStream>,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = vibrate ) ]
    ///The `vibrate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn vibrate_with_duration(this: &Navigator, duration: u32) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = vibrate ) ]
    ///The `vibrate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn vibrate_with_pattern(this: &Navigator, pattern: &::wasm_bindgen::JsValue) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = registerContentHandler ) ]
    ///The `registerContentHandler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerContentHandler)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn register_content_handler(
        this: &Navigator,
        mime_type: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Navigator" , js_name = registerProtocolHandler ) ]
    ///The `registerProtocolHandler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn register_protocol_handler(
        this: &Navigator,
        scheme: &str,
        url: &str,
        title: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Navigator" , js_name = taintEnabled ) ]
    ///The `taintEnabled()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)
    ///
    ///*This API requires the following crate features to be activated: `Navigator`*
    pub fn taint_enabled(this: &Navigator) -> bool;

}
