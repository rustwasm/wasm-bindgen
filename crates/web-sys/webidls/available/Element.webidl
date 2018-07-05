/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dom.spec.whatwg.org/#element and
 * http://domparsing.spec.whatwg.org/ and
 * http://dev.w3.org/csswg/cssom-view/ and
 * http://www.w3.org/TR/selectors-api/
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

interface Element : Node {
  [Constant]
  readonly attribute DOMString? namespaceURI;
  [Constant]
  readonly attribute DOMString? prefix;
  [Constant]
  readonly attribute DOMString localName;

  // Not [Constant] because it depends on which document we're in
  [Pure]
  readonly attribute DOMString tagName;

  [CEReactions, Pure]
           attribute DOMString id;
  [CEReactions, Pure]
           attribute DOMString className;
  [Constant, PutForwards=value]
  readonly attribute DOMTokenList classList;

  [SameObject]
  readonly attribute NamedNodeMap attributes;
  [Pure]
  sequence<DOMString> getAttributeNames();
  [Pure]
  DOMString? getAttribute(DOMString name);
  [Pure]
  DOMString? getAttributeNS(DOMString? namespace, DOMString localName);
  [CEReactions, NeedsSubjectPrincipal=NonSystem, Throws]
  boolean toggleAttribute(DOMString name, optional boolean force);
  [CEReactions, NeedsSubjectPrincipal=NonSystem, Throws]
  void setAttribute(DOMString name, DOMString value);
  [CEReactions, NeedsSubjectPrincipal=NonSystem, Throws]
  void setAttributeNS(DOMString? namespace, DOMString name, DOMString value);
  [CEReactions, Throws]
  void removeAttribute(DOMString name);
  [CEReactions, Throws]
  void removeAttributeNS(DOMString? namespace, DOMString localName);
  [Pure]
  boolean hasAttribute(DOMString name);
  [Pure]
  boolean hasAttributeNS(DOMString? namespace, DOMString localName);
  [Pure]
  boolean hasAttributes();

  [Throws, Pure]
  Element? closest(DOMString selector);

  [Throws, Pure]
  boolean matches(DOMString selector);
  [Throws, Pure, BinaryName="matches"]
  boolean webkitMatchesSelector(DOMString selector);

  [Pure]
  HTMLCollection getElementsByTagName(DOMString localName);
  [Throws, Pure]
  HTMLCollection getElementsByTagNameNS(DOMString? namespace, DOMString localName);
  [Pure]
  HTMLCollection getElementsByClassName(DOMString classNames);
  [ChromeOnly, Pure]
  sequence<Element> getElementsWithGrid();

  [CEReactions, Throws, Pure]
  Element? insertAdjacentElement(DOMString where, Element element); // historical

  [Throws]
  void insertAdjacentText(DOMString where, DOMString data); // historical

  /**
   * The ratio of font-size-inflated text font size to computed font
   * size for this element. This will query the element for its primary frame,
   * and then use this to get font size inflation information about the frame.
   * This will be 1.0 if font size inflation is not enabled, and -1.0 if an
   * error occurred during the retrieval of the font size inflation.
   *
   * @note The font size inflation ratio that is returned is actually the
   *       font size inflation data for the element's _primary frame_, not the
   *       element itself, but for most purposes, this should be sufficient.
   */
  [ChromeOnly]
  readonly attribute float fontSizeInflation;

  // Selectors API
  /**
   * Returns whether this element would be selected by the given selector
   * string.
   *
   * See <http://dev.w3.org/2006/webapi/selectors-api2/#matchesselector>
   */
  [Throws, Pure, BinaryName="matches"]
  boolean mozMatchesSelector(DOMString selector);

  // Pointer events methods.
  [Throws, Pref="dom.w3c_pointer_events.enabled"]
  void setPointerCapture(long pointerId);

  [Throws, Pref="dom.w3c_pointer_events.enabled"]
  void releasePointerCapture(long pointerId);

  [Pref="dom.w3c_pointer_events.enabled"]
  boolean hasPointerCapture(long pointerId);

  // Proprietary extensions
  /**
   * Set this during a mousedown event to grab and retarget all mouse events
   * to this element until the mouse button is released or releaseCapture is
   * called. If retargetToElement is true, then all events are targetted at
   * this element. If false, events can also fire at descendants of this
   * element.
   *
   */
  void setCapture(optional boolean retargetToElement = false);

  /**
   * If this element has captured the mouse, release the capture. If another
   * element has captured the mouse, this method has no effect.
   */
  void releaseCapture();

  /*
   * Chrome-only version of setCapture that works outside of a mousedown event.
   */
  [ChromeOnly]
  void setCaptureAlways(optional boolean retargetToElement = false);

  // Mozilla extensions

  // Obsolete methods.
  Attr? getAttributeNode(DOMString name);
  [CEReactions, Throws]
  Attr? setAttributeNode(Attr newAttr);
  [CEReactions, Throws]
  Attr? removeAttributeNode(Attr oldAttr);
  Attr? getAttributeNodeNS(DOMString? namespaceURI, DOMString localName);
  [CEReactions, Throws]
  Attr? setAttributeNodeNS(Attr newAttr);

  [ChromeOnly]
  /**
   * Scrolls the element by (dx, dy) CSS pixels without doing any
   * layout flushing.
   */
  boolean scrollByNoFlush(long dx, long dy);

  // Support reporting of Flexbox properties
  /**
   * If this element has a display:flex or display:inline-flex style,
   * this property returns an object with computed values for flex
   * properties, as well as a property that exposes the flex lines
   * in this container.
   */
  [ChromeOnly, Pure]
  Flex? getAsFlexContainer();

  // Support reporting of Grid properties
  /**
   * If this element has a display:grid or display:inline-grid style,
   * this property returns an object with computed values for grid
   * tracks and lines.
   */
  [ChromeOnly, Pure]
  sequence<Grid> getGridFragments();

  [ChromeOnly]
  DOMMatrixReadOnly getTransformToAncestor(Element ancestor);
  [ChromeOnly]
  DOMMatrixReadOnly getTransformToParent();
  [ChromeOnly]
  DOMMatrixReadOnly getTransformToViewport();
};

// http://dev.w3.org/csswg/cssom-view/
enum ScrollLogicalPosition { "start", "center", "end", "nearest" };
dictionary ScrollIntoViewOptions : ScrollOptions {
  ScrollLogicalPosition block = "start";
  ScrollLogicalPosition inline = "nearest";
};

// http://dev.w3.org/csswg/cssom-view/#extensions-to-the-element-interface
partial interface Element {
  DOMRectList getClientRects();
  DOMRect getBoundingClientRect();

  // scrolling
  void scrollIntoView(optional (boolean or ScrollIntoViewOptions) arg);
  // None of the CSSOM attributes are [Pure], because they flush
           attribute long scrollTop;   // scroll on setting
           attribute long scrollLeft;  // scroll on setting
  readonly attribute long scrollWidth;
  readonly attribute long scrollHeight;

  void scroll(unrestricted double x, unrestricted double y);
  void scroll(optional ScrollToOptions options);
  void scrollTo(unrestricted double x, unrestricted double y);
  void scrollTo(optional ScrollToOptions options);
  void scrollBy(unrestricted double x, unrestricted double y);
  void scrollBy(optional ScrollToOptions options);
  // mozScrollSnap is used by chrome to perform scroll snapping after the
  // user performs actions that may affect scroll position
  // mozScrollSnap is deprecated, to be replaced by a web accessible API, such
  // as an extension to the ScrollOptions dictionary.  See bug 1137937.
  [ChromeOnly] void mozScrollSnap();

  readonly attribute long clientTop;
  readonly attribute long clientLeft;
  readonly attribute long clientWidth;
  readonly attribute long clientHeight;

  // Mozilla specific stuff
  /* The minimum/maximum offset that the element can be scrolled to
     (i.e., the value that scrollLeft/scrollTop would be clamped to if they were
     set to arbitrarily large values. */
  [ChromeOnly] readonly attribute long scrollTopMin;
               readonly attribute long scrollTopMax;
  [ChromeOnly] readonly attribute long scrollLeftMin;
               readonly attribute long scrollLeftMax;
};

// http://domparsing.spec.whatwg.org/#extensions-to-the-element-interface
partial interface Element {
  [CEReactions, SetterNeedsSubjectPrincipal=NonSystem, Pure, SetterThrows, GetterCanOOM, TreatNullAs=EmptyString]
  attribute DOMString innerHTML;
  [CEReactions, Pure,SetterThrows,TreatNullAs=EmptyString]
  attribute DOMString outerHTML;
  [CEReactions, Throws]
  void insertAdjacentHTML(DOMString position, DOMString text);
};

// http://www.w3.org/TR/selectors-api/#interface-definitions
partial interface Element {
  [Throws, Pure]
  Element?  querySelector(DOMString selectors);
  [Throws, Pure]
  NodeList  querySelectorAll(DOMString selectors);
};

// https://dom.spec.whatwg.org/#dictdef-shadowrootinit
dictionary ShadowRootInit {
  required ShadowRootMode mode;
};

// https://dom.spec.whatwg.org/#element
partial interface Element {
  // Shadow DOM v1
  [Throws, Func="nsDocument::IsShadowDOMEnabled"]
  ShadowRoot attachShadow(ShadowRootInit shadowRootInitDict);
  [BinaryName="shadowRootByMode", Func="nsDocument::IsShadowDOMEnabled"]
  readonly attribute ShadowRoot? shadowRoot;

  [ChromeOnly, Func="nsDocument::IsShadowDOMEnabled", BinaryName="shadowRoot"]
  readonly attribute ShadowRoot? openOrClosedShadowRoot;

  [BinaryName="assignedSlotByMode", Func="nsDocument::IsShadowDOMEnabled"]
  readonly attribute HTMLSlotElement? assignedSlot;
  [CEReactions, Unscopable, SetterThrows, Func="nsDocument::IsShadowDOMEnabled"]
           attribute DOMString slot;
};

Element implements ChildNode;
Element implements NonDocumentTypeChildNode;
Element implements ParentNode;
Element implements Animatable;
Element implements GeometryUtils;

// https://fullscreen.spec.whatwg.org/#api
partial interface Element {
  [Throws, Func="nsDocument::IsUnprefixedFullscreenEnabled", NeedsCallerType]
  void requestFullscreen();
  [Throws, BinaryName="requestFullscreen", NeedsCallerType]
  void mozRequestFullScreen();
};

// https://w3c.github.io/pointerlock/#extensions-to-the-element-interface
partial interface Element {
  [NeedsCallerType]
  void requestPointerLock();
};
