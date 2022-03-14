// Import our JS shim and initialize it, executing the start function when it's
// ready.
import init from './pkg/wasm2js.js';
import {run} from './pkg/wasm2js.js';

// Will print "Hello World" because run() has "wasm_bindgen(start)"
// call init with input='js' so that it doesn't try to load the wasm
init('js');
// manually call run again to print another Hello World
run();