/* -*- Mode: IDL; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Some parts of WebGPU.webidl need to be pulled into a different file due to a codegen
 * bug/missing support.
 */

dictionary WebGPUBufferBinding {
    WebGPUBuffer buffer;
    u32 offset;
    u32 size;
};
