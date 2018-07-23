//@ts-check
// For more comments about what's going on here, check out the `hello_world`
// example
const events = require("events");

const rust = require("./node_module");

class MyEmitter extends events.EventEmitter {};

const jsEmitter = new MyEmitter();

console.log("created emitter");

jsEmitter.on('jsevent', function(a, b) {
    console.log(["jsevent", a, b]);
});

//rust.create_event(jsEmitter);

//jsEmitter.emit("rsevent");

rust.emit_event(jsEmitter);

const rsEmitter = rust.get_emitter();

rsEmitter.on('jsevent', function(a, b) {
    console.log(["jsevent", a, b]);
});

//rust.create_event(rsEmitter);

rust.emit_event(rsEmitter);

rsEmitter.emit("rsEmitter");
