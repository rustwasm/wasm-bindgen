/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is:
 * https://html.spec.whatwg.org/multipage/webappapis.html#windoworworkerglobalscope-mixin
 * https://fetch.spec.whatwg.org/#fetch-method
 * https://w3c.github.io/webappsec-secure-contexts/#monkey-patching-global-object
 * https://w3c.github.io/ServiceWorker/#self-caches
 */

// https://html.spec.whatwg.org/multipage/webappapis.html#windoworworkerglobalscope-mixin
[NoInterfaceObject, Exposed=(Window,Worker)]
interface WindowOrWorkerGlobalScope {
  [Replaceable] readonly attribute USVString origin;

  // base64 utility methods
  [Throws]
  DOMString btoa(DOMString btoa);
  [Throws]
  DOMString atob(DOMString atob);

  // timers
  // NOTE: We're using overloads where the spec uses a union.  Should
  // be black-box the same.
  [Throws]
  long setTimeout(Function handler, optional long timeout = 0, any... arguments);
  [Throws]
  long setTimeout(DOMString handler, optional long timeout = 0, any... unused);
  void clearTimeout(optional long handle = 0);
  [Throws]
  long setInterval(Function handler, optional long timeout = 0, any... arguments);
  [Throws]
  long setInterval(DOMString handler, optional long timeout = 0, any... unused);
  void clearInterval(optional long handle = 0);

  // ImageBitmap
  [Throws]
  Promise<ImageBitmap> createImageBitmap(ImageBitmapSource aImage);
  [Throws]
  Promise<ImageBitmap> createImageBitmap(ImageBitmapSource aImage, long aSx, long aSy, long aSw, long aSh);
};

// https://fetch.spec.whatwg.org/#fetch-method
partial interface WindowOrWorkerGlobalScope {
  [NewObject, NeedsCallerType]
  Promise<Response> fetch(RequestInfo input, optional RequestInit init);
};

// https://w3c.github.io/webappsec-secure-contexts/#monkey-patching-global-object
partial interface WindowOrWorkerGlobalScope {
  readonly attribute boolean isSecureContext;
};

// http://w3c.github.io/IndexedDB/#factory-interface
partial interface WindowOrWorkerGlobalScope {
   // readonly attribute IDBFactory indexedDB;
   [Throws]
   readonly attribute IDBFactory? indexedDB;
};

// https://w3c.github.io/ServiceWorker/#self-caches
partial interface WindowOrWorkerGlobalScope {
  [Throws, Func="mozilla::dom::DOMPrefs::DOMCachesEnabled", SameObject]
  readonly attribute CacheStorage caches;
};

// Mozilla extensions
partial interface WindowOrWorkerGlobalScope {
  // Extensions to ImageBitmap bits.
  // Bug 1141979 - [FoxEye] Extend ImageBitmap with interfaces to access its
  // underlying image data
  //
  // Note:
  // Overloaded functions cannot have different "extended attributes",
  // so I cannot add preference on the extended version of createImageBitmap().
  // To work around, I will then check the preference at run time and throw if
  // the preference is set to be false.
  [Throws]
  Promise<ImageBitmap> createImageBitmap(ImageBitmapSource aImage, long aOffset, long aLength, ImageBitmapFormat aFormat, ImagePixelLayout aLayout);
};
