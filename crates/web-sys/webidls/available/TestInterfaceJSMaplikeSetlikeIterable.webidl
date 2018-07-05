/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceMaplike {
  maplike<DOMString, long>;
  void setInternal(DOMString aKey, long aValue);
  void clearInternal();
  boolean deleteInternal(DOMString aKey);
  boolean hasInternal(DOMString aKey);
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceMaplikeObject {
  readonly maplike<DOMString, TestInterfaceMaplike>;
  void setInternal(DOMString aKey);
  void clearInternal();
  boolean deleteInternal(DOMString aKey);
  boolean hasInternal(DOMString aKey);
};

[Pref="dom.expose_test_interfaces",
 JSImplementation="@mozilla.org/dom/test-interface-js-maplike;1",
 Constructor()]
interface TestInterfaceJSMaplike {
  readonly maplike<DOMString, long>;
  void setInternal(DOMString aKey, long aValue);
  void clearInternal();
  boolean deleteInternal(DOMString aKey);
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceSetlike {
  setlike<DOMString>;
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceSetlikeNode {
  setlike<Node>;
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceIterableSingle {
  iterable<long>;
  getter long(unsigned long index);
  readonly attribute unsigned long length;
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceIterableDouble {
  iterable<DOMString, DOMString>;
};

[Constructor(),
 Pref="dom.expose_test_interfaces"]
interface TestInterfaceIterableDoubleUnion {
  iterable<DOMString, (DOMString or long)>;
};

