/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.w3.org/TR/2012/WD-dom-20120105/
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

interface Principal;
interface URI;

interface Node : EventTarget {
  const unsigned short ELEMENT_NODE = 1;
  const unsigned short ATTRIBUTE_NODE = 2; // historical
  const unsigned short TEXT_NODE = 3;
  const unsigned short CDATA_SECTION_NODE = 4; // historical
  const unsigned short ENTITY_REFERENCE_NODE = 5; // historical
  const unsigned short ENTITY_NODE = 6; // historical
  const unsigned short PROCESSING_INSTRUCTION_NODE = 7;
  const unsigned short COMMENT_NODE = 8;
  const unsigned short DOCUMENT_NODE = 9;
  const unsigned short DOCUMENT_TYPE_NODE = 10;
  const unsigned short DOCUMENT_FRAGMENT_NODE = 11;
  const unsigned short NOTATION_NODE = 12; // historical
  [Constant]
  readonly attribute unsigned short nodeType;
  [Pure]
  readonly attribute DOMString nodeName;

  [Pure, Throws, NeedsCallerType, BinaryName="baseURIFromJS"]
  readonly attribute DOMString? baseURI;

  [Pure, BinaryName=getComposedDoc]
  readonly attribute boolean isConnected;
  [Pure]
  readonly attribute Document? ownerDocument;
  [Pure]
  Node getRootNode(optional GetRootNodeOptions options);
  [Pure]
  readonly attribute Node? parentNode;
  [Pure]
  readonly attribute Element? parentElement;
  [Pure]
  boolean hasChildNodes();
  [SameObject]
  readonly attribute NodeList childNodes;
  [Pure]
  readonly attribute Node? firstChild;
  [Pure]
  readonly attribute Node? lastChild;
  [Pure]
  readonly attribute Node? previousSibling;
  [Pure]
  readonly attribute Node? nextSibling;

  [CEReactions, SetterThrows, Pure]
           attribute DOMString? nodeValue;
  [CEReactions, SetterThrows, GetterCanOOM,
   SetterNeedsSubjectPrincipal=NonSystem, Pure]
           attribute DOMString? textContent;
  [CEReactions, Throws]
  Node insertBefore(Node node, Node? child);
  [CEReactions, Throws]
  Node appendChild(Node node);
  [CEReactions, Throws]
  Node replaceChild(Node node, Node child);
  [CEReactions, Throws]
  Node removeChild(Node child);
  [CEReactions]
  void normalize();

  [CEReactions, Throws]
  Node cloneNode(optional boolean deep = false);
  [Pure]
  boolean isSameNode(Node? node);
  [Pure]
  boolean isEqualNode(Node? node);

  const unsigned short DOCUMENT_POSITION_DISCONNECTED = 0x01;
  const unsigned short DOCUMENT_POSITION_PRECEDING = 0x02;
  const unsigned short DOCUMENT_POSITION_FOLLOWING = 0x04;
  const unsigned short DOCUMENT_POSITION_CONTAINS = 0x08;
  const unsigned short DOCUMENT_POSITION_CONTAINED_BY = 0x10;
  const unsigned short DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC = 0x20; // historical
  [Pure]
  unsigned short compareDocumentPosition(Node other);
  [Pure]
  boolean contains(Node? other);

  [Pure]
  DOMString? lookupPrefix(DOMString? namespace);
  [Pure]
  DOMString? lookupNamespaceURI(DOMString? prefix);
  [Pure]
  boolean isDefaultNamespace(DOMString? namespace);

  // Mozilla-specific stuff
  [ChromeOnly]
  readonly attribute Principal nodePrincipal;
  [ChromeOnly]
  readonly attribute URI? baseURIObject;
  [ChromeOnly]
  DOMString generateXPath();

  /**
   * This method provides a fast-path for the Fluent localization system to
   * bypass the slowdowns in performance during initial document translation.
   * The slowdowns are specific to XBL+Stylo.
   * To learn more, see bug 1441037.
   *
   * The API is designed to fit into the DOMLocalization flow with minimal
   * overhead, which dictates much of its signature.
   * It takes the following steps:
   *
   * 1) The API can be called at any point on any DOM element and it
   *    synchronously scans the element subtree for all children with
   *    `data-l10n-id` attribute set.
   *
   * 2) Next, the API collects all of the l10n attributes
   *    (l10n-id, l10n-args and l10n-attrs), and passes them to the
   *    callback function together with three `Element` properties:
   *      `name` - name of the element as lowercase
   *      `namespaceURI` - namespace URI
   *      `type` - the type prop of the element (used for input sanitization)
   *
   * 3) The callback function is responsible for (asynchronously) collecting
   *    the translations for all l10n id+args pairs, sanitizing them and then
   *    return them back to this API.
   *
   * 4) The API takes the list of elements collected in step (1) and their
   *    translations and applies all of the translation values onto
   *    the elements.
   *
   * 5) The API returns a list with empty slots for all translated elements
   *    and references to elements that could not be translated.
   *
   * 6) The JS handles the translations of remaining elements.
   *
   *
   * Through the whole cycle, the API uses the same list of elements and
   * corresponding translations. It means that after step (1), the element
   * at index 1 will match the l10nData at index 1, translations at index 1
   * and in the final return list, the element will be also stored at index 1
   * or the slot will be empty if the translations was applied on the C++ side.
   *
   * Note: There are several reasons why the JS callback may pass undefined for
   *       a given element including missing translation, or the need to
   *       translate the element using DOM Overlays.
   *
   *
   * Example of use from JS:
   *
   * async function translateFragment(frag) {
   *   let untranslatedElements = await frag.localize(
   *     async cb(l10nItems) => {                          // 1
   *       let trans = await getTranslations(l10nItems);   // 2
   *       return trans;
   *     }
   *   );
   *
   *   annotateMissingTranslations(untranslatedElements);  // 3
   * }
   *
   * [1] l10nItems == [
   *       {
   *         l10nId: "key1",
   *         l10nArgs: null,
   *         l10nAttrs: null,
   *         name: "button"
   *         namespaceURI: "..."
   *         type: null
   *       },
   *       {
   *         l10nId: "key2",
   *         l10nArgs: {unreadCount: 5},
   *         l10nAttrs: null,
   *         name: "label"
   *         namespaceURI: "..."
   *         type: null
   *       },
   *       {
   *         l10nId: "key3",
   *         l10nArgs: null,
   *         l10nAttrs: "title",
   *         name: "window"
   *         namespaceURI: "..."
   *         type: null
   *       },
   *     ]
   * [2] trans == [
   *       {value: "Key 1", attributes: {accesskey: "K"} },
   *       undefined,
   *       {value: null, attributes: {title: "Unread emails: 5"} },
   *     ]
   * [3] untranslatedElements == [
   *       ,
   *       <label>
   *       ,
   *     ]
   *
   * For exact dictionary structures, see `L10nUtils.webidl`.
   */
  [ChromeOnly, Throws]
  Promise<void> localize(L10nCallback l10nCallback);

#ifdef ACCESSIBILITY
  [Func="mozilla::dom::AccessibleNode::IsAOMEnabled", SameObject]
  readonly attribute AccessibleNode? accessibleNode;
#endif
};

dictionary GetRootNodeOptions {
  boolean composed = false;
};
