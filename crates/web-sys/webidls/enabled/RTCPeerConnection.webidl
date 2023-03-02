/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://w3c.github.io/webrtc-pc/#interface-definition
 */

callback RTCSessionDescriptionCallback = undefined (RTCSessionDescriptionInit description);
callback RTCPeerConnectionErrorCallback = undefined (DOMException error);
callback RTCStatsCallback = undefined (RTCStatsReport report);

enum RTCSignalingState {
    "stable",
    "have-local-offer",
    "have-remote-offer",
    "have-local-pranswer",
    "have-remote-pranswer",
    "closed"
};

enum RTCIceGatheringState {
    "new",
    "gathering",
    "complete"
};

enum RTCIceConnectionState {
    "new",
    "checking",
    "connected",
    "completed",
    "failed",
    "disconnected",
    "closed"
};

enum RTCPeerConnectionState {
  "closed",
  "failed",
  "disconnected",
  "new",
  "connecting",
  "connected"
};

dictionary RTCDataChannelInit {
  boolean        ordered = true;
  unsigned short maxPacketLifeTime;
  unsigned short maxRetransmits;
  DOMString      protocol = "";
  boolean        negotiated = false;
  unsigned short id;

  // These are deprecated due to renaming in the spec, but still supported for Fx53
  unsigned short maxRetransmitTime;
};

dictionary RTCOfferAnswerOptions {
//  boolean voiceActivityDetection = true; // TODO: support this (Bug 1184712)
};

dictionary RTCAnswerOptions : RTCOfferAnswerOptions {
};

dictionary RTCOfferOptions : RTCOfferAnswerOptions {
  boolean offerToReceiveVideo;
  boolean offerToReceiveAudio;
  boolean iceRestart = false;
};

dictionary RTCLocalSessionDescriptionInit {
  RTCSdpType type;
  DOMString sdp = "";
};

[Exposed=Window]
interface RTCPeerConnection : EventTarget  {
  constructor(optional RTCConfiguration configuration = {});
  Promise<RTCSessionDescriptionInit> createOffer(optional RTCOfferOptions options = {});
  Promise<RTCSessionDescriptionInit> createAnswer(optional RTCAnswerOptions options = {});
  Promise<undefined> setLocalDescription(optional RTCLocalSessionDescriptionInit description = {});
  readonly attribute RTCSessionDescription? localDescription;
  readonly attribute RTCSessionDescription? currentLocalDescription;
  readonly attribute RTCSessionDescription? pendingLocalDescription;
  Promise<undefined> setRemoteDescription(RTCSessionDescriptionInit description);
  readonly attribute RTCSessionDescription? remoteDescription;
  readonly attribute RTCSessionDescription? currentRemoteDescription;
  readonly attribute RTCSessionDescription? pendingRemoteDescription;
  Promise<undefined> addIceCandidate(optional RTCIceCandidateInit candidate = {});
  readonly attribute RTCSignalingState signalingState;
  readonly attribute RTCIceGatheringState iceGatheringState;
  readonly attribute RTCIceConnectionState iceConnectionState;
  readonly attribute RTCPeerConnectionState connectionState;
  readonly attribute boolean? canTrickleIceCandidates;
  undefined restartIce();
  RTCConfiguration getConfiguration();
  undefined setConfiguration(optional RTCConfiguration configuration = {});
  undefined close();
  attribute EventHandler onnegotiationneeded;
  attribute EventHandler onicecandidate;
  attribute EventHandler onicecandidateerror;
  attribute EventHandler onsignalingstatechange;
  attribute EventHandler oniceconnectionstatechange;
  attribute EventHandler onicegatheringstatechange;
  attribute EventHandler onconnectionstatechange;

  // Legacy Interface Extensions
  // Supporting the methods in this section is optional.
  // If these methods are supported
  // they must be implemented as defined
  // in section "Legacy Interface Extensions"
  Promise<undefined> createOffer(RTCSessionDescriptionCallback successCallback,
                            RTCPeerConnectionErrorCallback failureCallback,
                            optional RTCOfferOptions options = {});
  Promise<undefined> setLocalDescription(RTCLocalSessionDescriptionInit description,
                                    VoidFunction successCallback,
                                    RTCPeerConnectionErrorCallback failureCallback);
  Promise<undefined> createAnswer(RTCSessionDescriptionCallback successCallback,
                             RTCPeerConnectionErrorCallback failureCallback);
  Promise<undefined> setRemoteDescription(RTCSessionDescriptionInit description,
                                     VoidFunction successCallback,
                                     RTCPeerConnectionErrorCallback failureCallback);
  Promise<undefined> addIceCandidate(RTCIceCandidateInit candidate,
                                VoidFunction successCallback,
                                RTCPeerConnectionErrorCallback failureCallback);
};

partial interface RTCPeerConnection {
  sequence<RTCRtpSender> getSenders();
  sequence<RTCRtpReceiver> getReceivers();
  sequence<RTCRtpTransceiver> getTransceivers();
  RTCRtpSender addTrack(MediaStreamTrack track, MediaStream... streams);
  undefined removeTrack(RTCRtpSender sender);
  RTCRtpTransceiver addTransceiver((MediaStreamTrack or DOMString) trackOrKind,
                                   optional RTCRtpTransceiverInit init = {});
  attribute EventHandler ontrack;
};