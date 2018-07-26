/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-history-interface
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

enum ScrollRestoration { "auto", "manual" };

interface History {
  [Throws]
  readonly attribute unsigned long length;
  [Throws]
  attribute ScrollRestoration scrollRestoration;
  [Throws]
  readonly attribute any state;
  [Throws]
  void go(optional long delta = 0);
  [Throws]
  void back();
  [Throws]
  void forward();
  [Throws]
  void pushState(any data, DOMString title, optional DOMString? url = null);
  [Throws]
  void replaceState(any data, DOMString title, optional DOMString? url = null);
};
