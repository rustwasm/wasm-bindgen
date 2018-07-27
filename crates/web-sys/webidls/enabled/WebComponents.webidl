/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dvcs.w3.org/hg/webcomponents/raw-file/tip/spec/custom/index.html
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

callback LifecycleConnectedCallback = void();
callback LifecycleDisconnectedCallback = void();
callback LifecycleAdoptedCallback = void(Document? oldDocument,
                                         Document? newDocment);
callback LifecycleAttributeChangedCallback = void(DOMString attrName,
                                                  DOMString? oldValue,
                                                  DOMString? newValue,
                                                  DOMString? namespaceURI);

dictionary LifecycleCallbacks {
  LifecycleConnectedCallback connectedCallback;
  LifecycleDisconnectedCallback disconnectedCallback;
  LifecycleAdoptedCallback adoptedCallback;
  LifecycleAttributeChangedCallback attributeChangedCallback;
};
