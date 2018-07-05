/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//
// These dictionaries are in a separate WebIDL file to avoid circular include
// problems. One of the dictionary includes a union as a member, so that
// dictionary's header needs to include UnionTypes.h. But the API in
// TestInterfaceJS also declares a union of dictionaries, so _that_
// dictionary's header needs to be included _by_ UnionTypes.h. The solution
// is to separate those two dictionaries into separate header files.
//

dictionary TestInterfaceJSDictionary2 {
  object innerObject;
};

dictionary TestInterfaceJSDictionary {
  TestInterfaceJSDictionary2 innerDictionary;
  object objectMember;
  any anyMember;
  (object or DOMString) objectOrStringMember;
  sequence<any> anySequenceMember;
};

