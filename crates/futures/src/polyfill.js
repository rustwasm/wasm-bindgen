/*
 * The polyfill was kindly borrowed from https://github.com/tc39/proposal-atomics-wait-async
 */

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Author: Lars T Hansen, lhansen@mozilla.com
 */

/* Polyfill for Atomics.waitAsync() for web browsers.
 *
 * Any kind of agent that is able to create a new Worker can use this polyfill.
 *
 * Load this file in all agents that will use Atomics.waitAsync.
 *
 * Agents that don't call Atomics.waitAsync need do nothing special.
 *
 * Any kind of agent can wake another agent that is sleeping in
 * Atomics.waitAsync by just calling Atomics.wake for the location being slept
 * on, as normal.
 *
 * The implementation is not completely faithful to the proposed semantics: in
 * the case where an agent first asyncWaits and then waits on the same location:
 * when it is woken, the two waits will be woken in order, while in the real
 * semantics, the sync wait will be woken first.
 *
 * In this polyfill Atomics.waitAsync is not very fast.
 */

/* Implementation:
 *
 * For every wait we fork off a Worker to perform the wait.  Workers are reused
 * when possible.  The worker communicates with its parent using postMessage.
 */

const helperCode = `
onmessage = function (ev) {
    try {
        switch (ev.data[0]) {
          case 'wait': {
            let [_, ia, index, value, timeout] = ev.data;
            let result = Atomics.wait(ia, index, value, timeout)
            postMessage(['ok', result]);
            break;
          }
          default: {
            throw new Error("Wrong message sent to wait helper: " + ev.data.join(','));
          }
        }
    } catch (e) {
        console.log("Exception in wait helper");
        postMessage(['error', 'Exception']);
    }
}
`;

const helpers = [];

function allocHelper() {
    if (helpers.length > 0) {
        return helpers.pop();
    }
    return new Worker("data:application/javascript," + encodeURIComponent(helperCode));
}

function freeHelper(h) {
    helpers.push(h);
}

// Atomics.waitAsync always returns a promise.  Throws standard errors
// for parameter validation.  The promise is resolved with a string as from
// Atomics.wait, or, in the case something went completely wrong, it is
// rejected with an error string.
export function waitAsync(ia, index, value, timeout = Infinity) {
    if (typeof ia != "object"
        || !(ia instanceof Int32Array)
        || !(ia.buffer instanceof SharedArrayBuffer)
    ) {
        throw new TypeError("Expected shared memory");
    }

    // Range checking for the index.

    ia[index];

    // Optimization, avoid the helper thread in this common case.

    if (Atomics.load(ia, index) !== value) {
        return Promise.resolve("not-equal");
    }

    // General case, we must wait.

    return new Promise(function (resolve, reject) {
        const h = allocHelper();
        h.onmessage = function (ev) {
            // Free the helper early so that it can be reused if the resolution
            // needs a helper.
            freeHelper(h);
            switch (ev.data[0]) {
                case 'ok':
                    resolve(ev.data[1]);
                    break;
                case 'error':
                    // Note, rejection is not in the spec, it is an artifact of the polyfill.
                    // The helper already printed an error to the console.
                    reject(ev.data[1]);
                    break;
            }
        };

        // It's possible to do better here if the ia is already known to the
        // helper.  In that case we can communicate the other data through
        // shared memory and wake the agent.  And it is possible to make ia
        // known to the helper by waking it with a special value so that it
        // checks its messages, and then posting the ia to the helper.  Some
        // caching / decay scheme is useful no doubt, to improve performance
        // and avoid leaks.
        //
        // In the event we wake the helper directly, we can micro-wait here
        // for a quick result.  We'll need to restructure some code to make
        // that work out properly, and some synchronization is necessary for
        // the helper to know that we've picked up the result and no
        // postMessage is necessary.

        h.postMessage(['wait', ia, index, value, timeout]);
    })
}
