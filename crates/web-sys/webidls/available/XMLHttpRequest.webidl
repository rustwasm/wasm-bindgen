/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://xhr.spec.whatwg.org/#interface-xmlhttprequest
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

interface InputStream;
interface MozChannel;
interface IID;

enum XMLHttpRequestResponseType {
  "",
  "arraybuffer",
  "blob",
  "document",
  "json",
  "text",

  // Mozilla-specific stuff
  "moz-chunked-arraybuffer",
};

/**
 * Parameters for instantiating an XMLHttpRequest. They are passed as an
 * optional argument to the constructor:
 *
 *  new XMLHttpRequest({anon: true, system: true});
 */
dictionary MozXMLHttpRequestParameters
{
  /**
   * If true, the request will be sent without cookie and authentication
   * headers.
   */
  boolean mozAnon = false;

  /**
   * If true, the same origin policy will not be enforced on the request.
   */
  boolean mozSystem = false;
};

[Constructor(optional MozXMLHttpRequestParameters params),
 // There are apparently callers, specifically CoffeeScript, who do
 // things like this:
 //   c = new(window.ActiveXObject || XMLHttpRequest)("Microsoft.XMLHTTP")
 // To handle that, we need a constructor that takes a string.
 Constructor(DOMString ignored),
 Exposed=(Window,DedicatedWorker,SharedWorker)]
interface XMLHttpRequest : XMLHttpRequestEventTarget {
  // event handler
  attribute EventHandler onreadystatechange;

  // states
  const unsigned short UNSENT = 0;
  const unsigned short OPENED = 1;
  const unsigned short HEADERS_RECEIVED = 2;
  const unsigned short LOADING = 3;
  const unsigned short DONE = 4;

  readonly attribute unsigned short readyState;

  // request
  [Throws]
  void open(ByteString method, USVString url);
  [Throws]
  void open(ByteString method, USVString url, boolean async,
            optional USVString? user=null, optional USVString? password=null);
  [Throws]
  void setRequestHeader(ByteString header, ByteString value);

  [SetterThrows]
  attribute unsigned long timeout;

  [SetterThrows]
  attribute boolean withCredentials;

  [Throws]
  readonly attribute XMLHttpRequestUpload upload;

  [Throws]
  void send(optional (Document or BodyInit)? body = null);

  [Throws]
  void abort();

  // response
  readonly attribute USVString responseURL;

  [Throws]
  readonly attribute unsigned short status;

  [Throws]
  readonly attribute ByteString statusText;

  [Throws]
  ByteString? getResponseHeader(ByteString header);

  [Throws]
  ByteString getAllResponseHeaders();

  [Throws]
  void overrideMimeType(DOMString mime);

  [SetterThrows]
  attribute XMLHttpRequestResponseType responseType;
  [Throws]
  readonly attribute any response;
  [Cached, Pure, Throws]
  readonly attribute USVString? responseText;

  [Throws, Exposed=Window]
  readonly attribute Document? responseXML;

  // Mozilla-specific stuff

  [ChromeOnly, SetterThrows]
  attribute boolean mozBackgroundRequest;

  [ChromeOnly, Exposed=Window]
  readonly attribute MozChannel? channel;

  [Throws, ChromeOnly, Exposed=Window]
  any getInterface(IID iid);

  [ChromeOnly, Exposed=Window]
  void setOriginAttributes(optional OriginAttributesDictionary originAttributes);

  [ChromeOnly, Throws]
  void sendInputStream(InputStream body);

  // Only works on MainThread.
  // Its permanence is to be evaluated in bug 1368540 for Firefox 60.
  [ChromeOnly]
  readonly attribute unsigned short errorCode;

  readonly attribute boolean mozAnon;
  readonly attribute boolean mozSystem;
};
