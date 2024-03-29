/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnectioniceerrorevent
 */

[Exposed=Window]
interface RTCPeerConnectionIceErrorEvent : Event {
  constructor(DOMString type, RTCPeerConnectionIceErrorEventInit eventInitDict);
  readonly attribute DOMString? address;
  readonly attribute unsigned short? port;
  readonly attribute DOMString url;
  readonly attribute unsigned short errorCode;
  readonly attribute USVString errorText;
};
