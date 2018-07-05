/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dom.spec.whatwg.org/#interface-childnode
 */

[NoInterfaceObject]
interface ChildNode {
  [CEReactions, Throws, Unscopable]
  void before((Node or DOMString)... nodes);
  [CEReactions, Throws, Unscopable]
  void after((Node or DOMString)... nodes);
  [CEReactions, Throws, Unscopable]
  void replaceWith((Node or DOMString)... nodes);
  [CEReactions, Unscopable]
  void remove();
};

[NoInterfaceObject]
interface NonDocumentTypeChildNode {
  [Pure]
  readonly attribute Element? previousElementSibling;
  [Pure]
  readonly attribute Element? nextElementSibling;
};
