/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://domparsing.spec.whatwg.org/#the-domparser-interface
 */

interface Principal;
interface URI;
interface InputStream;

enum SupportedType {
  "text/html",
  "text/xml",
  "application/xml",
  "application/xhtml+xml",
  "image/svg+xml"
};

// the latter is Mozilla-specific
[Constructor]
interface DOMParser {
  [NewObject, Throws]
  Document parseFromString(DOMString str, SupportedType type);

  // Mozilla-specific stuff
  [NewObject, Throws, ChromeOnly]
  Document parseFromBuffer(sequence<octet> buf, SupportedType type);
  [NewObject, Throws, ChromeOnly]
  Document parseFromBuffer(Uint8Array buf, SupportedType type);
  [NewObject, Throws, ChromeOnly]
  Document parseFromStream(InputStream stream, DOMString? charset,
                           long contentLength, SupportedType type);
  // Can be used to allow a DOMParser to parse XUL/XBL no matter what
  // principal it's using for the document.
  [ChromeOnly]
  void forceEnableXULXBL();
};

