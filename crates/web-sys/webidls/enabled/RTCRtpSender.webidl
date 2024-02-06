/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://w3c.github.io/webrtc-pc/#rtcrtpsender-interface
 */

enum RTCPriorityType {
  "very-low",
  "low",
  "medium",
  "high"
};

enum RTCDegradationPreference {
  "maintain-framerate",
  "maintain-resolution",
  "balanced"
};

dictionary RTCRtxParameters {
  unsigned long ssrc;
};

dictionary RTCFecParameters {
  unsigned long ssrc;
};

dictionary RTCRtpEncodingParameters {
  unsigned long            ssrc;
  RTCRtxParameters         rtx;
  RTCFecParameters         fec;
  boolean                  active;
  RTCPriorityType          priority;
  unsigned long            maxBitrate;
  RTCDegradationPreference degradationPreference = "balanced";
  DOMString                rid;
  float                    scaleResolutionDownBy = 1.0;
};

dictionary RTCRtpHeaderExtensionParameters {
  DOMString      uri;
  unsigned short id;
  boolean        encrypted;
};

dictionary RTCRtcpParameters {
  DOMString cname;
  boolean   reducedSize;
};

dictionary RTCRtpCodec {
  required  DOMString mimeType;
  required  unsigned long clockRate;
  unsigned  short channels;
  DOMString sdpFmtpLine;
};

dictionary RTCRtpCodecParameters : RTCRtpCodec {
  required octet payloadType;
};

dictionary RTCRtpCodecCapability : RTCRtpCodec {
};

dictionary RTCRtpHeaderExtensionCapability {
  required DOMString uri;
};

dictionary RTCRtpCapabilities {
  required sequence<RTCRtpCodecCapability>           codecs;
  required sequence<RTCRtpHeaderExtensionCapability> headerExtensions;
};

[Pref="media.peerconnection.enabled",
 JSImplementation="@mozilla.org/dom/rtpsender;1"]
interface RTCRtpSender {
  readonly attribute MediaStreamTrack? track;
  Promise<undefined> setParameters (optional RTCRtpParameters parameters);
  RTCRtpParameters getParameters();
  Promise<undefined> replaceTrack(MediaStreamTrack? withTrack);
  Promise<RTCStatsReport> getStats();
  static RTCRtpCapabilities? getCapabilities(DOMString kind);
  [Pref="media.peerconnection.dtmf.enabled"]
  readonly attribute RTCDTMFSender? dtmf;
  // Ugh, can't use a ChromeOnly attribute sequence<MediaStream>...
  [ChromeOnly]
  sequence<MediaStream> getStreams();
  [ChromeOnly]
  undefined setStreams(sequence<MediaStream> streams);
  [ChromeOnly]
  undefined setTrack(MediaStreamTrack? track);
  [ChromeOnly]
  undefined checkWasCreatedByPc(RTCPeerConnection pc);
};
