/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://www.w3.org/TR/webtransport/
 *
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and Opera Software ASA.
 * You are granted a license to use, reproduce and create derivative works of this document.
 */

[Exposed=(Window,Worker), SecureContext]
interface WebTransportDatagramDuplexStream {
  readonly attribute ReadableStream readable;
  readonly attribute WritableStream writable;

  readonly attribute unsigned long maxDatagramSize;
  attribute unrestricted double incomingMaxAge;
  attribute unrestricted double outgoingMaxAge;
  attribute unrestricted double incomingHighWaterMark;
  attribute unrestricted double outgoingHighWaterMark;
};

[Exposed=(Window,Worker), SecureContext]
interface WebTransport {
  constructor(USVString url, optional WebTransportOptions options = {});

  Promise<WebTransportStats> getStats();
  readonly attribute Promise<undefined> ready;
  readonly attribute WebTransportReliabilityMode reliability;
  readonly attribute WebTransportCongestionControl congestionControl;
  readonly attribute Promise<WebTransportCloseInfo> closed;
  readonly attribute Promise<undefined> draining;
  undefined close(optional WebTransportCloseInfo closeInfo = {});

  readonly attribute WebTransportDatagramDuplexStream datagrams;

  Promise<WebTransportBidirectionalStream> createBidirectionalStream(
      optional WebTransportSendStreamOptions options = {});
  /* a ReadableStream of WebTransportBidirectionalStream objects */
  readonly attribute ReadableStream incomingBidirectionalStreams;

  Promise<WebTransportSendStream> createUnidirectionalStream(
      optional WebTransportSendStreamOptions options = {});
  /* a ReadableStream of WebTransportReceiveStream objects */
  readonly attribute ReadableStream incomingUnidirectionalStreams;
};

enum WebTransportReliabilityMode {
  "pending",
  "reliable-only",
  "supports-unreliable",
};

dictionary WebTransportHash {
  DOMString algorithm;
  BufferSource value;
};

dictionary WebTransportOptions {
  boolean allowPooling = false;
  boolean requireUnreliable = false;
  sequence<WebTransportHash> serverCertificateHashes;
  WebTransportCongestionControl congestionControl = "default";
};

enum WebTransportCongestionControl {
  "default",
  "throughput",
  "low-latency",
};

dictionary WebTransportCloseInfo {
  unsigned long closeCode = 0;
  USVString reason = "";
};

dictionary WebTransportSendStreamOptions {
  long long? sendOrder = null;
};

dictionary WebTransportStats {
  DOMHighResTimeStamp timestamp;
  unsigned long long bytesSent;
  unsigned long long packetsSent;
  unsigned long long packetsLost;
  unsigned long numOutgoingStreamsCreated;
  unsigned long numIncomingStreamsCreated;
  unsigned long long bytesReceived;
  unsigned long long packetsReceived;
  DOMHighResTimeStamp smoothedRtt;
  DOMHighResTimeStamp rttVariation;
  DOMHighResTimeStamp minRtt;
  WebTransportDatagramStats datagrams;
};

dictionary WebTransportDatagramStats {
  DOMHighResTimeStamp timestamp;
  unsigned long long expiredOutgoing;
  unsigned long long droppedIncoming;
  unsigned long long lostOutgoing;
};

[Exposed=(Window,Worker), SecureContext, Transferable]
interface WebTransportSendStream : WritableStream {
  Promise<WebTransportSendStreamStats> getStats();
};

dictionary WebTransportSendStreamStats {
  DOMHighResTimeStamp timestamp;
  unsigned long long bytesWritten;
  unsigned long long bytesSent;
  unsigned long long bytesAcknowledged;
};

[Exposed=(Window,Worker), SecureContext, Transferable]
interface WebTransportReceiveStream : ReadableStream {
  Promise<WebTransportReceiveStreamStats> getStats();
};

dictionary WebTransportReceiveStreamStats {
  DOMHighResTimeStamp timestamp;
  unsigned long long bytesReceived;
  unsigned long long bytesRead;
};

[Exposed=(Window,Worker), SecureContext]
interface WebTransportBidirectionalStream {
  readonly attribute WebTransportReceiveStream readable;
  readonly attribute WebTransportSendStream writable;
};

[Exposed=(Window,Worker), Serializable, SecureContext]
interface WebTransportError : DOMException {
  constructor(optional DOMString message = "", optional WebTransportErrorOptions options = {});

  readonly attribute WebTransportErrorSource source;
  readonly attribute octet? streamErrorCode;
};

dictionary WebTransportErrorOptions {
  WebTransportErrorSource source = "stream";
  [Clamp] octet? streamErrorCode = null;
};

enum WebTransportErrorSource {
  "stream",
  "session",
};
