/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

[Func="IsChromeOrXBL"]
interface BoxObject {
  readonly attribute Element? element;

  readonly attribute long x;
  readonly attribute long y;
  [Throws]
  readonly attribute long screenX;
  [Throws]
  readonly attribute long screenY;
  readonly attribute long width;
  readonly attribute long height;

  nsISupports? getPropertyAsSupports(DOMString propertyName);
  void setPropertyAsSupports(DOMString propertyName, nsISupports value);
  [Throws]
  DOMString? getProperty(DOMString propertyName);
  void setProperty(DOMString propertyName, DOMString propertyValue);
  void removeProperty(DOMString propertyName);

  // for stepping through content in the expanded dom with box-ordinal-group order
  readonly attribute Element? parentBox;
  readonly attribute Element? firstChild;
  readonly attribute Element? lastChild;
  readonly attribute Element? nextSibling;
  readonly attribute Element? previousSibling;
};
