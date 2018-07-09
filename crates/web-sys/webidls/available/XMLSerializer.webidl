/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://domparsing.spec.whatwg.org/#the-xmlserializer-interface
 */

interface OutputStream;

[Constructor]
interface XMLSerializer {
  /**
   * The subtree rooted by the specified element is serialized to
   * a string.
   *
   * @param root The root of the subtree to be serialized. This could
   *             be any node, including a Document.
   * @returns The serialized subtree in the form of a Unicode string
   */
  [Throws]
  DOMString serializeToString(Node root);

  // Mozilla-specific stuff
  /**
   * The subtree rooted by the specified element is serialized to
   * a byte stream using the character set specified.
   * @param root The root of the subtree to be serialized. This could
   *             be any node, including a Document.
   * @param stream The byte stream to which the subtree is serialized.
   * @param charset The name of the character set to use for the encoding
   *                to a byte stream.  If this string is empty and root is
   *                a document, the document's character set will be used.
   */
  [Throws, ChromeOnly]
  void serializeToStream(Node root, OutputStream stream, DOMString? charset);
};

