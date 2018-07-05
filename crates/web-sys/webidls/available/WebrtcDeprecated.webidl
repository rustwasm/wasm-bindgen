/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * This file includes all the deprecated mozRTC prefixed interfaces.
 *
 * The declaration of each should match the declaration of the real, unprefixed
 * interface.  These aliases will be removed at some point (Bug 1155923).
 */

[Deprecated="WebrtcDeprecatedPrefix",
 Pref="media.peerconnection.enabled",
 JSImplementation="@mozilla.org/dom/rtcicecandidate;1",
 Constructor(optional RTCIceCandidateInit candidateInitDict)]
interface mozRTCIceCandidate : RTCIceCandidate {};

[Deprecated="WebrtcDeprecatedPrefix",
 Pref="media.peerconnection.enabled",
 JSImplementation="@mozilla.org/dom/peerconnection;1",
 Constructor (optional RTCConfiguration configuration,
              optional object? constraints)]
interface mozRTCPeerConnection : RTCPeerConnection {};

[Deprecated="WebrtcDeprecatedPrefix",
 Pref="media.peerconnection.enabled",
 JSImplementation="@mozilla.org/dom/rtcsessiondescription;1",
 Constructor(optional RTCSessionDescriptionInit descriptionInitDict)]
interface mozRTCSessionDescription : RTCSessionDescription {};
