/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dev.w3.org/2011/webrtc/editor/webrtc.html#idl-def-RTCSessionDescription
 */

enum RTCSdpType {
  "offer",
  "pranswer",
  "answer",
  "rollback"
};

dictionary RTCSessionDescriptionInit {
  required RTCSdpType type;
  DOMString sdp = "";
};

[Exposed=Window]
interface RTCSessionDescription {
  constructor(RTCSessionDescriptionInit descriptionInitDict);
  readonly attribute RTCSdpType type;
  readonly attribute DOMString sdp;
  [Default] object toJSON();
};
