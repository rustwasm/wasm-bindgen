/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.w3.org/TR/2012/WD-dom-20120105/
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */


dictionary EventListenerOptions {
  boolean capture = false;
  /* Setting to true make the listener be added to the system group. */
  [Func="ThreadSafeIsChromeOrXBL"]
  boolean mozSystemGroup = false;
};

dictionary AddEventListenerOptions : EventListenerOptions {
  boolean passive;
  boolean once = false;
};

[Constructor,
 Exposed=(Window,Worker,WorkerDebugger,AudioWorklet,System)]
interface EventTarget {
  /* Passing null for wantsUntrusted means "default behavior", which
     differs in content and chrome.  In content that default boolean
     value is true, while in chrome the default boolean value is
     false. */
  [Throws]
  void addEventListener(DOMString type,
                        EventListener? listener,
                        optional (AddEventListenerOptions or boolean) options,
                        optional boolean? wantsUntrusted = null);
  [Throws]
  void removeEventListener(DOMString type,
                           EventListener? listener,
                           optional (EventListenerOptions or boolean) options);
  [Throws, NeedsCallerType]
  boolean dispatchEvent(Event event);
};

// Mozilla extensions for use by JS-implemented event targets to
// implement on* properties.
partial interface EventTarget {
  // The use of [TreatNonCallableAsNull] here is a bit of a hack: it just makes
  // the codegen check whether the type involved is either
  // [TreatNonCallableAsNull] or [TreatNonObjectAsNull] and if it is handle it
  // accordingly.  In particular, it will NOT actually treat a non-null
  // non-callable object as null here.
  [ChromeOnly, Throws]
  void setEventHandler(DOMString type,
                       [TreatNonCallableAsNull] EventHandler handler);

  [ChromeOnly]
  EventHandler getEventHandler(DOMString type);
};

// Mozilla extension to make firing events on event targets from
// chrome easier.  This returns the window which can be used to create
// events to fire at this EventTarget, or null if there isn't one.
partial interface EventTarget {
  [ChromeOnly, Exposed=(Window,System), BinaryName="ownerGlobalForBindings"]
  readonly attribute WindowProxy? ownerGlobal;
};
