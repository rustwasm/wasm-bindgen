/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

[Func="IsChromeOrXBL"]
interface TreeColumn {
  readonly attribute Element element;

  readonly attribute TreeColumns? columns;

  [Throws]
  readonly attribute long x;
  [Throws]
  readonly attribute long width;

  readonly attribute DOMString id;
  readonly attribute long index;

  readonly attribute boolean primary;
  readonly attribute boolean cycler;
  readonly attribute boolean editable;
  readonly attribute boolean selectable;

  const short TYPE_TEXT                = 1;
  const short TYPE_CHECKBOX            = 2;
  const short TYPE_PROGRESSMETER       = 3;
  const short TYPE_PASSWORD            = 4;
  readonly attribute short type;

  TreeColumn? getNext();
  TreeColumn? getPrevious();

  [Throws]
  void invalidate();
};
