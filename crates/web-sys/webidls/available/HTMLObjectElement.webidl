/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-object-element
 * http://www.whatwg.org/specs/web-apps/current-work/#HTMLObjectElement-partial
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

// http://www.whatwg.org/specs/web-apps/current-work/#the-object-element
[HTMLConstructor, NeedResolve]
interface HTMLObjectElement : HTMLElement {
  [CEReactions, Pure, SetterThrows]
           attribute DOMString data;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString type;
  [CEReactions, Pure, SetterThrows]
           attribute boolean typeMustMatch;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString name;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString useMap;
  [Pure]
  readonly attribute HTMLFormElement? form;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString width;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString height;
  // Not pure: can trigger about:blank instantiation
  [NeedsSubjectPrincipal]
  readonly attribute Document? contentDocument;
  // Not pure: can trigger about:blank instantiation
  [NeedsSubjectPrincipal]
  readonly attribute WindowProxy? contentWindow;

  readonly attribute boolean willValidate;
  readonly attribute ValidityState validity;
  [Throws]
  readonly attribute DOMString validationMessage;
  boolean checkValidity();
  boolean reportValidity();
  void setCustomValidity(DOMString error);
};

// http://www.whatwg.org/specs/web-apps/current-work/#HTMLObjectElement-partial
partial interface HTMLObjectElement {
  [CEReactions, Pure, SetterThrows]
           attribute DOMString align;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString archive;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString code;
  [CEReactions, Pure, SetterThrows]
           attribute boolean declare;
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long hspace;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString standby;
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long vspace;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString codeBase;
  [CEReactions, Pure, SetterThrows]
           attribute DOMString codeType;

  [CEReactions, TreatNullAs=EmptyString, Pure, SetterThrows]
           attribute DOMString border;
};

partial interface HTMLObjectElement {
  // GetSVGDocument
  [NeedsSubjectPrincipal]
  Document? getSVGDocument();
};

[NoInterfaceObject]
interface MozObjectLoadingContent {
  // Mirrored chrome-only scriptable nsIObjectLoadingContent methods.  Please
  // make sure to update this list if nsIObjectLoadingContent changes.  Also,
  // make sure everything on here is [ChromeOnly].
  [ChromeOnly]
  const unsigned long TYPE_LOADING     = 0;
  [ChromeOnly]
  const unsigned long TYPE_IMAGE       = 1;
  [ChromeOnly]
  const unsigned long TYPE_PLUGIN      = 2;
  [ChromeOnly]
  const unsigned long TYPE_FAKE_PLUGIN = 3;
  [ChromeOnly]
  const unsigned long TYPE_DOCUMENT    = 4;
  [ChromeOnly]
  const unsigned long TYPE_NULL        = 5;

  // The content type is not supported (e.g. plugin not installed)
  [ChromeOnly]
  const unsigned long PLUGIN_UNSUPPORTED          = 0;
  // Showing alternate content
  [ChromeOnly]
  const unsigned long PLUGIN_ALTERNATE            = 1;
  // The plugin exists, but is disabled
  [ChromeOnly]
  const unsigned long PLUGIN_DISABLED             = 2;
  // The plugin is blocklisted and disabled
  [ChromeOnly]
  const unsigned long PLUGIN_BLOCKLISTED          = 3;
  // The plugin is considered outdated, but not disabled
  [ChromeOnly]
  const unsigned long PLUGIN_OUTDATED             = 4;
  // The plugin has crashed
  [ChromeOnly]
  const unsigned long PLUGIN_CRASHED              = 5;
  // Suppressed by security policy
  [ChromeOnly]
  const unsigned long PLUGIN_SUPPRESSED           = 6;
  // Blocked by content policy
  [ChromeOnly]
  const unsigned long PLUGIN_USER_DISABLED        = 7;
  /// ** All values >= PLUGIN_CLICK_TO_PLAY are plugin placeholder types that
  ///    would be replaced by a real plugin if activated (playPlugin())
  /// ** Furthermore, values >= PLUGIN_CLICK_TO_PLAY and
  ///    <= PLUGIN_VULNERABLE_NO_UPDATE are click-to-play types.
  // The plugin is disabled until the user clicks on it
  [ChromeOnly]
  const unsigned long PLUGIN_CLICK_TO_PLAY        = 8;
  // The plugin is vulnerable (update available)
  [ChromeOnly]
  const unsigned long PLUGIN_VULNERABLE_UPDATABLE = 9;
  // The plugin is vulnerable (no update available)
  [ChromeOnly]
  const unsigned long PLUGIN_VULNERABLE_NO_UPDATE = 10;

  /**
   * The actual mime type (the one we got back from the network
   * request) for the element.
   */
  [ChromeOnly]
  readonly attribute DOMString actualType;

  /**
   * Gets the type of the content that's currently loaded. See
   * the constants above for the list of possible values.
   */
  [ChromeOnly]
  readonly attribute unsigned long displayedType;

  /**
   * Gets the content type that corresponds to the give MIME type.  See the
   * constants above for the list of possible values.  If nothing else fits,
   * TYPE_NULL will be returned.
   */
  [ChromeOnly]
  unsigned long getContentTypeForMIMEType(DOMString aMimeType);


  [ChromeOnly]
  sequence<MozPluginParameter> getPluginAttributes();

  [ChromeOnly]
  sequence<MozPluginParameter> getPluginParameters();

  /**
   * This method will play a plugin that has been stopped by the click-to-play
   * feature.
   */
  [ChromeOnly, Throws, NeedsCallerType]
  void playPlugin();

  /**
   * Forces a re-evaluation and reload of the tag, optionally invalidating its
   * click-to-play state.  This can be used when the MIME type that provides a
   * type has changed, for instance, to force the tag to re-evalulate the
   * handler to use.
   */
  [ChromeOnly, Throws]
  void reload(boolean aClearActivation);

  /**
   * This attribute will return true if the current content type has been
   * activated, either explicitly or by passing checks that would have it be
   * click-to-play.
   */
  [ChromeOnly]
  readonly attribute boolean activated;

  /**
   * The URL of the data/src loaded in the object. This may be null (i.e.
   * an <embed> with no src).
   */
  [ChromeOnly]
  readonly attribute URI? srcURI;

  [ChromeOnly]
  readonly attribute unsigned long defaultFallbackType;

  [ChromeOnly]
  readonly attribute unsigned long pluginFallbackType;

  /**
   * If this object currently owns a running plugin, regardless of whether or
   * not one is pending spawn/despawn.
   */
  [ChromeOnly]
  readonly attribute boolean hasRunningPlugin;

  /**
   * Disable the use of fake plugins and reload the tag if necessary
   */
  [ChromeOnly, Throws]
  void skipFakePlugins();

  [ChromeOnly, Throws, NeedsCallerType]
  readonly attribute unsigned long runID;
};

/**
 * Name:Value pair type used for passing parameters to NPAPI or javascript
 * plugins.
 */
dictionary MozPluginParameter {
  DOMString name = "";
  DOMString value = "";
};

HTMLObjectElement implements MozImageLoadingContent;
HTMLObjectElement implements MozFrameLoaderOwner;
HTMLObjectElement implements MozObjectLoadingContent;
