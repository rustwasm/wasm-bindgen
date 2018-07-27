/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-navigator-object
 * http://www.w3.org/TR/tracking-dnt/
 * http://www.w3.org/TR/geolocation-API/#geolocation_interface
 * http://www.w3.org/TR/battery-status/#navigatorbattery-interface
 * http://www.w3.org/TR/vibration/#vibration-interface
 * http://www.w3.org/2012/sysapps/runtime/#extension-to-the-navigator-interface-1
 * https://dvcs.w3.org/hg/gamepad/raw-file/default/gamepad.html#navigator-interface-extension
 * http://www.w3.org/TR/beacon/#sec-beacon-method
 * https://html.spec.whatwg.org/#navigatorconcurrenthardware
 * http://wicg.github.io/netinfo/#extensions-to-the-navigator-interface
 * https://w3c.github.io/webappsec-credential-management/#framework-credential-management
 * https://w3c.github.io/webdriver/webdriver-spec.html#interface
 * https://wicg.github.io/media-capabilities/#idl-index
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

// http://www.whatwg.org/specs/web-apps/current-work/#the-navigator-object
[HeaderFile="Navigator.h"]
interface Navigator {
  // objects implementing this interface also implement the interfaces given below
};
Navigator implements NavigatorID;
Navigator implements NavigatorLanguage;
Navigator implements NavigatorOnLine;
Navigator implements NavigatorContentUtils;
Navigator implements NavigatorStorageUtils;
Navigator implements NavigatorConcurrentHardware;
Navigator implements NavigatorStorage;
Navigator implements NavigatorAutomationInformation;

[NoInterfaceObject, Exposed=(Window,Worker)]
interface NavigatorID {
  // WebKit/Blink/Trident/Presto support this (hardcoded "Mozilla").
  [Constant, Cached, Throws]
  readonly attribute DOMString appCodeName; // constant "Mozilla"
  [Constant, Cached, NeedsCallerType]
  readonly attribute DOMString appName;
  [Constant, Cached, Throws, NeedsCallerType]
  readonly attribute DOMString appVersion;
  [Constant, Cached, Throws, NeedsCallerType]
  readonly attribute DOMString platform;
  [Pure, Cached, Throws, NeedsCallerType]
  readonly attribute DOMString userAgent;
  [Constant, Cached]
  readonly attribute DOMString product; // constant "Gecko"

  // Everyone but WebKit/Blink supports this.  See bug 679971.
  [Exposed=Window]
  boolean taintEnabled(); // constant false
};

[NoInterfaceObject, Exposed=(Window,Worker)]
interface NavigatorLanguage {

  // These two attributes are cached because this interface is also implemented
  // by Workernavigator and this way we don't have to go back to the
  // main-thread from the worker thread anytime we need to retrieve them. They
  // are updated when pref intl.accept_languages is changed.

  [Pure, Cached]
  readonly attribute DOMString? language;
  [Pure, Cached, Frozen]
  readonly attribute sequence<DOMString> languages;
};

[NoInterfaceObject, Exposed=(Window,Worker)]
interface NavigatorOnLine {
  readonly attribute boolean onLine;
};

[NoInterfaceObject]
interface NavigatorContentUtils {
  // content handler registration
  [Throws, Func="nsGlobalWindowInner::RegisterProtocolHandlerAllowedForContext"]
  void registerProtocolHandler(DOMString scheme, DOMString url, DOMString title);
  [Pref="dom.registerContentHandler.enabled", Throws]
  void registerContentHandler(DOMString mimeType, DOMString url, DOMString title);
  // NOT IMPLEMENTED
  //DOMString isProtocolHandlerRegistered(DOMString scheme, DOMString url);
  //DOMString isContentHandlerRegistered(DOMString mimeType, DOMString url);
  //void unregisterProtocolHandler(DOMString scheme, DOMString url);
  //void unregisterContentHandler(DOMString mimeType, DOMString url);
};

[SecureContext, NoInterfaceObject, Exposed=(Window,Worker)]
interface NavigatorStorage {
  [Func="mozilla::dom::DOMPrefs::StorageManagerEnabled"]
  readonly attribute StorageManager storage;
};

[NoInterfaceObject]
interface NavigatorStorageUtils {
  // NOT IMPLEMENTED
  //void yieldForStorageUpdates();
};

partial interface Navigator {
  [Throws]
  readonly attribute Permissions permissions;
};

// Things that definitely need to be in the spec and and are not for some
// reason.  See https://www.w3.org/Bugs/Public/show_bug.cgi?id=22406
partial interface Navigator {
  [Throws]
  readonly attribute MimeTypeArray mimeTypes;
  [Throws]
  readonly attribute PluginArray plugins;
};

// http://www.w3.org/TR/tracking-dnt/ sort of
partial interface Navigator {
  readonly attribute DOMString doNotTrack;
};

// http://www.w3.org/TR/geolocation-API/#geolocation_interface
[NoInterfaceObject]
interface NavigatorGeolocation {
  [Throws, Pref="geo.enabled"]
  readonly attribute Geolocation geolocation;
};
Navigator implements NavigatorGeolocation;

// http://www.w3.org/TR/battery-status/#navigatorbattery-interface
partial interface Navigator {
  // ChromeOnly to prevent web content from fingerprinting users' batteries.
  [Throws, ChromeOnly, Pref="dom.battery.enabled"]
  Promise<BatteryManager> getBattery();
};

// http://www.w3.org/TR/vibration/#vibration-interface
partial interface Navigator {
    // We don't support sequences in unions yet
    //boolean vibrate ((unsigned long or sequence<unsigned long>) pattern);
    boolean vibrate(unsigned long duration);
    boolean vibrate(sequence<unsigned long> pattern);
};

// http://www.w3.org/TR/pointerevents/#extensions-to-the-navigator-interface
partial interface Navigator {
    [Pref="dom.w3c_pointer_events.enabled"]
    readonly attribute long maxTouchPoints;
};

// https://wicg.github.io/media-capabilities/#idl-index
[Exposed=Window]
partial interface Navigator {
  [SameObject, Func="mozilla::dom::MediaCapabilities::Enabled"]
  readonly attribute MediaCapabilities mediaCapabilities;
};

// Mozilla-specific extensions

// Chrome-only interface for Vibration API permission handling.
partial interface Navigator {
    /* Set permission state to device vibration.
     * @param permitted permission state (true for allowing vibration)
     * @param persistent make the permission session-persistent
     */
    [ChromeOnly]
    void setVibrationPermission(boolean permitted,
                                optional boolean persistent = true);
};

callback interface MozIdleObserver {
  // Time is in seconds and is read only when idle observers are added
  // and removed.
  readonly attribute unsigned long time;
  void onidle();
  void onactive();
};

partial interface Navigator {
  [Throws, Constant, Cached, NeedsCallerType]
  readonly attribute DOMString oscpu;
  // WebKit/Blink support this; Trident/Presto do not.
  readonly attribute DOMString vendor;
  // WebKit/Blink supports this (hardcoded ""); Trident/Presto do not.
  readonly attribute DOMString vendorSub;
  // WebKit/Blink supports this (hardcoded "20030107"); Trident/Presto don't
  readonly attribute DOMString productSub;
  // WebKit/Blink/Trident/Presto support this.
  readonly attribute boolean cookieEnabled;
  [Throws, Constant, Cached, NeedsCallerType]
  readonly attribute DOMString buildID;

  // WebKit/Blink/Trident/Presto support this.
  [Affects=Nothing, DependsOn=Nothing]
  boolean javaEnabled();

  /**
   * Navigator requests to add an idle observer to the existing window.
   */
  [Throws, ChromeOnly]
  void addIdleObserver(MozIdleObserver aIdleObserver);

  /**
   * Navigator requests to remove an idle observer from the existing window.
   */
  [Throws, ChromeOnly]
  void removeIdleObserver(MozIdleObserver aIdleObserver);
};

// NetworkInformation
partial interface Navigator {
  [Throws, Pref="dom.netinfo.enabled"]
  readonly attribute NetworkInformation connection;
};

// https://dvcs.w3.org/hg/gamepad/raw-file/default/gamepad.html#navigator-interface-extension
partial interface Navigator {
  [Throws, Pref="dom.gamepad.enabled"]
  sequence<Gamepad?> getGamepads();
};
partial interface Navigator {
  [Pref="dom.gamepad.test.enabled"]
  GamepadServiceTest requestGamepadServiceTest();
};

partial interface Navigator {
  [Throws, Pref="dom.vr.enabled"]
  Promise<sequence<VRDisplay>> getVRDisplays();
  // TODO: Use FrozenArray once available. (Bug 1236777)
  [Frozen, Cached, Pure, Pref="dom.vr.enabled"]
  readonly attribute sequence<VRDisplay> activeVRDisplays;
  [ChromeOnly, Pref="dom.vr.enabled"]
  readonly attribute boolean isWebVRContentDetected;
  [ChromeOnly, Pref="dom.vr.enabled"]
  readonly attribute boolean isWebVRContentPresenting;
  [ChromeOnly, Pref="dom.vr.enabled"]
  void requestVRPresentation(VRDisplay display);
};
partial interface Navigator {
  [Pref="dom.vr.test.enabled"]
  VRServiceTest requestVRServiceTest();
};

// http://webaudio.github.io/web-midi-api/#requestmidiaccess
partial interface Navigator {
  [Throws, Pref="dom.webmidi.enabled"]
  Promise<MIDIAccess> requestMIDIAccess(optional MIDIOptions options);
};

callback NavigatorUserMediaSuccessCallback = void (MediaStream stream);
callback NavigatorUserMediaErrorCallback = void (MediaStreamError error);

partial interface Navigator {
  [Throws, Func="Navigator::HasUserMediaSupport"]
  readonly attribute MediaDevices mediaDevices;

  // Deprecated. Use mediaDevices.getUserMedia instead.
  [Deprecated="NavigatorGetUserMedia", Throws,
   Func="Navigator::HasUserMediaSupport",
   NeedsCallerType]
  void mozGetUserMedia(MediaStreamConstraints constraints,
                       NavigatorUserMediaSuccessCallback successCallback,
                       NavigatorUserMediaErrorCallback errorCallback);
};

// nsINavigatorUserMedia
callback MozGetUserMediaDevicesSuccessCallback = void (nsIVariant? devices);
partial interface Navigator {
  [Throws, ChromeOnly]
  void mozGetUserMediaDevices(MediaStreamConstraints constraints,
                              MozGetUserMediaDevicesSuccessCallback onsuccess,
                              NavigatorUserMediaErrorCallback onerror,
                              // The originating innerWindowID is needed to
                              // avoid calling the callbacks if the window has
                              // navigated away. It is optional only as legacy.
                              optional unsigned long long innerWindowID = 0,
                              // The callID is needed in case of multiple
                              // concurrent requests to find the right one.
                              // It is optional only as legacy.
                              // TODO: Rewrite to not need this method anymore,
                              // now that devices are enumerated earlier.
                              optional DOMString callID = "");
};

// Service Workers/Navigation Controllers
partial interface Navigator {
  [Func="ServiceWorkerContainer::IsEnabled", SameObject]
  readonly attribute ServiceWorkerContainer serviceWorker;
};

partial interface Navigator {
  [Throws, Pref="beacon.enabled"]
  boolean sendBeacon(DOMString url,
                     optional BodyInit? data = null);
};

partial interface Navigator {
  [Throws, Pref="dom.presentation.enabled", SameObject]
  readonly attribute Presentation? presentation;
};

partial interface Navigator {
  [NewObject, Func="mozilla::dom::TCPSocket::ShouldTCPSocketExist"]
  readonly attribute LegacyMozTCPSocket mozTCPSocket;
};

partial interface Navigator {
  [NewObject]
  Promise<MediaKeySystemAccess>
  requestMediaKeySystemAccess(DOMString keySystem,
                              sequence<MediaKeySystemConfiguration> supportedConfigurations);
};

[NoInterfaceObject, Exposed=(Window,Worker)]
interface NavigatorConcurrentHardware {
  readonly attribute unsigned long long hardwareConcurrency;
};

// https://w3c.github.io/webappsec-credential-management/#framework-credential-management
partial interface Navigator {
  [Pref="security.webauth.webauthn", SecureContext, SameObject]
  readonly attribute CredentialsContainer credentials;
};

// https://w3c.github.io/webdriver/webdriver-spec.html#interface
[NoInterfaceObject]
interface NavigatorAutomationInformation {
  [Pref="dom.webdriver.enabled"]
  readonly attribute boolean webdriver;
};
