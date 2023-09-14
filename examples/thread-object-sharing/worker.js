/** @type {import('./pkg/thread_object_sharing')} */
import init, { OpaqueObject } from "./pkg/thread_object_sharing.js";

/**
 * @typedef {{type: 'load', memory: WebAssembly.Memory}} WorkerRequestLoad
 * @typedef {{type: 'start', opaqueObjectPtr: number}} WorkerRequestStart
 * @typedef {{type: 'stop'}} WorkerRequestStop
 * @typedef {{type: 'increment', 'incrementVal': number}} WorkerRequestIncrement
 * @typedef {WorkerRequestLoad | WorkerRequestStart | WorkerRequestStop | WorkerRequestIncrement} WorkerRequest
 */

/** @type {number | undefined} */
let intervalTimer = undefined;
let incrementVal = 10;

/** @type {OpaqueObject} */
const opaqueObject = Object.create(OpaqueObject.prototype);

function tick() {
    opaqueObject.add(incrementVal);
}

self.onmessage = async (event) => {
    /**
     * @type {WorkerRequest}
     */
    const request = event.data;

    switch (request.type) {
        case 'load':
            await init(undefined, request.memory);
            break;
        case 'start':
            opaqueObject.__wbg_ptr = request.opaqueObjectPtr;
            if (intervalTimer === undefined) {
                intervalTimer = self.setInterval(tick, 15);
                console.log('starting intervalTimer');
            }
            break;
        case 'stop':
            console.log('stopping intervalTimer');
            self.clearInterval(intervalTimer);
            intervalTimer = undefined;
            break;
        case 'increment':
            incrementVal = request.incrementVal;
            console.log(`New increment value: ${incrementVal}`);
            break;
    }
};
