/** @type {import('./pkg/thread_object_sharing')} */
import initWasm, { OpaqueObject } from "./pkg/thread_object_sharing.js";

// Create a few webworkers, leaving some breathing room so our computer doesn't die
const numWorkers = navigator.hardwareConcurrency / 2;
const workers = [];
for (let i = 0; i < numWorkers; ++i) {
    workers.push(new Worker(
        new URL('./worker.js', import.meta.url),
        { name: 'Call OpaqueObject.get() a bunch of times', type: 'module' },
    ));
}

// Load the webassembly and send our memory to the workers
const wasmInternals = await initWasm();
for (const worker of workers) {
    worker.postMessage({ type: 'load', memory: wasmInternals.memory });
}

const opaqueObject = new OpaqueObject(47);

// True iff webworkers are running
let running = false;

// Visual representation of opaqueObject.get()
const valueMeter = window.document.getElementById('value-meter');
const valueDisplayText = window.document.getElementById('value-display-text');

// Change the rate at which the webworkers increment opaqueObject.get()'s value
const incrementInput = window.document.getElementById('increment-input');
incrementInput.onchange = (event) => {
    const incrementVal = event.target.value;
    for (const worker of workers) {
        worker.postMessage({ type: 'increment', incrementVal });
    }
}

/**
 * Start the webworkers
 */
function start() {
    for (const worker of workers) {
        worker.postMessage({ type: 'start', opaqueObjectPtr: opaqueObject.__wbg_ptr });
    }
    running = true;
}

/**
 * Stop the webworkers
 */
function stop() {
    for (const worker of workers) {
        worker.postMessage({ type: 'stop' });
    }
    running = false;
}

// Clickable button to start and stop the webworkers
const toggleButton = window.document.getElementById('toggle-button');
toggleButton.onclick = () => {
    if (running) {
        stop();
    } else {
        start();
    }
    toggleButton.innerText = running ? 'stop' : 'start';
};
toggleButton.removeAttribute('disabled');
toggleButton.innerText = running ? 'stop' : 'start';

function render() {
    const val = opaqueObject.get();
    valueMeter.setAttribute('value', val);
    valueDisplayText.innerText = val.toString();

    window.requestAnimationFrame(render);
}
window.requestAnimationFrame(render);
