let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm = undefined;

    let WASM_VECTOR_LEN = 0;

    let cachedUint8ArrayMemory0 = null;

    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
            cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8ArrayMemory0;
    }

    const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

    const encodeString = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8ArrayMemory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
            ptr = realloc(ptr, len, offset, 1) >>> 0;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedDataViewMemory0 = null;

    function getDataViewMemory0() {
        if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
            cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
        }
        return cachedDataViewMemory0;
    }

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_export_5.set(idx, obj);
        return idx;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    }

    const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

    if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => {
        wasm.__wbindgen_export_7.get(state.dtor)(state.a, state.b)
    });

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_7.get(state.dtor)(a, state.b);
                    CLOSURE_DTORS.unregister(state);
                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches && builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    function takeFromExternrefTable0(idx) {
        const value = wasm.__wbindgen_export_5.get(idx);
        wasm.__externref_table_dealloc(idx);
        return value;
    }
    /**
     * Entry point invoked by `worker.js`, a bit of a hack but see the "TODO" above
     * about `worker.js` in general.
     * @param {number} ptr
     */
    __exports.child_entry_point = function(ptr) {
        const ret = wasm.child_entry_point(ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    };

    function _assertClass(instance, klass) {
        if (!(instance instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
    }
    function __wbg_adapter_48(arg0, arg1, arg2) {
        wasm.closure46_externref_shim(arg0, arg1, arg2);
    }

    function __wbg_adapter_51(arg0, arg1, arg2) {
        wasm.closure91_externref_shim(arg0, arg1, arg2);
    }

    function __wbg_adapter_144(arg0, arg1, arg2, arg3) {
        wasm.closure146_externref_shim(arg0, arg1, arg2, arg3);
    }

    const RenderingSceneFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_renderingscene_free(ptr >>> 0, 1));

    class RenderingScene {

        static __wrap(ptr) {
            ptr = ptr >>> 0;
            const obj = Object.create(RenderingScene.prototype);
            obj.__wbg_ptr = ptr;
            RenderingSceneFinalization.register(obj, obj.__wbg_ptr, obj);
            return obj;
        }

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            RenderingSceneFinalization.unregister(this);
            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_renderingscene_free(ptr, 0);
        }
        /**
         * Returns the JS promise object which resolves when the render is complete
         * @returns {Promise<any>}
         */
        promise() {
            const ret = wasm.renderingscene_promise(this.__wbg_ptr);
            return ret;
        }
        /**
         * Return a progressive rendering of the image so far
         * @returns {ImageData}
         */
        imageSoFar() {
            const ret = wasm.renderingscene_imageSoFar(this.__wbg_ptr);
            return ret;
        }
    }
    __exports.RenderingScene = RenderingScene;

    const SceneFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_scene_free(ptr >>> 0, 1));

    class Scene {

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            SceneFinalization.unregister(this);
            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_scene_free(ptr, 0);
        }
        /**
         * Creates a new scene from the JSON description in `object`, which we
         * deserialize here into an actual scene.
         * @param {any} object
         */
        constructor(object) {
            const ret = wasm.scene_new(object);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            this.__wbg_ptr = ret[0] >>> 0;
            SceneFinalization.register(this, this.__wbg_ptr, this);
            return this;
        }
        /**
         * Renders this scene with the provided concurrency and worker pool.
         *
         * This will spawn up to `concurrency` workers which are loaded from or
         * spawned into `pool`. The `RenderingScene` state contains information to
         * get notifications when the render has completed.
         * @param {number} concurrency
         * @param {WorkerPool} pool
         * @returns {RenderingScene}
         */
        render(concurrency, pool) {
            const ptr = this.__destroy_into_raw();
            _assertClass(pool, WorkerPool);
            const ret = wasm.scene_render(ptr, concurrency, pool.__wbg_ptr);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            return RenderingScene.__wrap(ret[0]);
        }
    }
    __exports.Scene = Scene;

    const WorkerPoolFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_workerpool_free(ptr >>> 0, 1));

    class WorkerPool {

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            WorkerPoolFinalization.unregister(this);
            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_workerpool_free(ptr, 0);
        }
        /**
         * Creates a new `WorkerPool` which immediately creates `initial` workers.
         *
         * The pool created here can be used over a long period of time, and it
         * will be initially primed with `initial` workers. Currently workers are
         * never released or gc'd until the whole pool is destroyed.
         *
         * # Errors
         *
         * Returns any error that may happen while a JS web worker is created and a
         * message is sent to it.
         * @param {number} initial
         */
        constructor(initial) {
            const ret = wasm.workerpool_new(initial);
            if (ret[2]) {
                throw takeFromExternrefTable0(ret[1]);
            }
            this.__wbg_ptr = ret[0] >>> 0;
            WorkerPoolFinalization.register(this, this.__wbg_ptr, this);
            return this;
        }
    }
    __exports.WorkerPool = WorkerPool;

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    function __wbg_get_imports() {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
            const ret = String(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbg_async_9ff6d9e405f13772 = function(arg0) {
            const ret = arg0.async;
            return ret;
        };
        imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
            const ret = arg0.buffer;
            return ret;
        };
        imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_data_432d9c3df2630942 = function(arg0) {
            const ret = arg0.data;
            return ret;
        };
        imports.wbg.__wbg_done_769e5ede4b31c67b = function(arg0) {
            const ret = arg0.done;
            return ret;
        };
        imports.wbg.__wbg_entries_3265d4158b33e5dc = function(arg0) {
            const ret = Object.entries(arg0);
            return ret;
        };
        imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        };
        imports.wbg.__wbg_get_67b2ba62fc30de12 = function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_get_b9b93047fe3cf45b = function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        };
        imports.wbg.__wbg_getwithrefkey_1dc361bd10053bfe = function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        };
        imports.wbg.__wbg_instanceof_ArrayBuffer_e14585432e3737fc = function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_instanceof_ErrorEvent_24a579ed4d838fe9 = function(arg0) {
            let result;
            try {
                result = arg0 instanceof ErrorEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_instanceof_MessageEvent_2e467ced55f682c9 = function(arg0) {
            let result;
            try {
                result = arg0 instanceof MessageEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_instanceof_Uint8Array_17156bcf118086a9 = function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_isArray_a1eab7e0d067391b = function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        };
        imports.wbg.__wbg_isSafeInteger_343e2beeeece1bb0 = function(arg0) {
            const ret = Number.isSafeInteger(arg0);
            return ret;
        };
        imports.wbg.__wbg_iterator_9a24c88df860dc65 = function() {
            const ret = Symbol.iterator;
            return ret;
        };
        imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
            const ret = arg0.length;
            return ret;
        };
        imports.wbg.__wbg_length_e2d2a49132c1b256 = function(arg0) {
            const ret = arg0.length;
            return ret;
        };
        imports.wbg.__wbg_log_2aaf3380b1303cf1 = function(arg0) {
            console.log(arg0);
        };
        imports.wbg.__wbg_log_8a764cae2094e649 = function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbg_message_d1685a448ba00178 = function(arg0, arg1) {
            const ret = arg1.message;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbg_new_23a2665fac83c611 = function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wbg_adapter_144(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = state0.b = 0;
            }
        };
        imports.wbg.__wbg_new_78feb108b6472713 = function() {
            const ret = new Array();
            return ret;
        };
        imports.wbg.__wbg_new_7a91e41fe43b3c92 = function(arg0) {
            const ret = new Uint8ClampedArray(arg0);
            return ret;
        };
        imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
            const ret = new Error();
            return ret;
        };
        imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        };
        imports.wbg.__wbg_new_b1a33e5095abf678 = function() { return handleError(function (arg0, arg1) {
            const ret = new Worker(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments) };
        imports.wbg.__wbg_new_e9a4a67dbababe57 = function(arg0) {
            const ret = new Int32Array(arg0);
            return ret;
        };
        imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        };
        imports.wbg.__wbg_newwithjsu8clampedarrayandsh_7f3fdc36fd8f9d7a = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new ImageData(arg0, arg1 >>> 0, arg2 >>> 0);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_next_25feadfc0913fea9 = function(arg0) {
            const ret = arg0.next;
            return ret;
        };
        imports.wbg.__wbg_next_6574e1a8a62d1055 = function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments) };
        imports.wbg.__wbg_of_4a05197bfc89556f = function(arg0, arg1, arg2) {
            const ret = Array.of(arg0, arg1, arg2);
            return ret;
        };
        imports.wbg.__wbg_postMessage_6edafa8f7b9c2f52 = function() { return handleError(function (arg0, arg1) {
            arg0.postMessage(arg1);
        }, arguments) };
        imports.wbg.__wbg_postMessage_83a8d58d3fcb6c13 = function() { return handleError(function (arg0, arg1) {
            arg0.postMessage(arg1);
        }, arguments) };
        imports.wbg.__wbg_push_737cfc8c1432c2c6 = function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        };
        imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5 = function(arg0) {
            queueMicrotask(arg0);
        };
        imports.wbg.__wbg_queueMicrotask_d3219def82552485 = function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        };
        imports.wbg.__wbg_resolve_4851785c9c5f573d = function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        };
        imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
            arg0.set(arg1, arg2 >>> 0);
        };
        imports.wbg.__wbg_setonerror_57eeef5feb01fe7a = function(arg0, arg1) {
            arg0.onerror = arg1;
        };
        imports.wbg.__wbg_setonmessage_5a885b16bdc6dca6 = function(arg0, arg1) {
            arg0.onmessage = arg1;
        };
        imports.wbg.__wbg_slice_8339a12068af339c = function(arg0, arg1, arg2) {
            const ret = arg0.slice(arg1 >>> 0, arg2 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        };
        imports.wbg.__wbg_then_44b73946d2fb3e7d = function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        };
        imports.wbg.__wbg_type_16f2b8031796512f = function(arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbg_value_cd1ffa7b1ab794f1 = function(arg0) {
            const ret = arg0.value;
            return ret;
        };
        imports.wbg.__wbg_value_dab73d3d5d4abaaf = function(arg0) {
            const ret = arg0.value;
            return ret;
        };
        imports.wbg.__wbg_waitAsync_61f0a081053dd3c2 = function(arg0, arg1, arg2) {
            const ret = Atomics.waitAsync(arg0, arg1 >>> 0, arg2);
            return ret;
        };
        imports.wbg.__wbg_waitAsync_7ce6c8a047c752c3 = function() {
            const ret = Atomics.waitAsync;
            return ret;
        };
        imports.wbg.__wbindgen_as_number = function(arg0) {
            const ret = +arg0;
            return ret;
        };
        imports.wbg.__wbindgen_boolean_get = function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
            return ret;
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = arg0.original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            const ret = false;
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper176 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 47, __wbg_adapter_48);
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper271 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 92, __wbg_adapter_51);
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper272 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 92, __wbg_adapter_51);
            return ret;
        };
        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return ret;
        };
        imports.wbg.__wbindgen_in = function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        };
        imports.wbg.__wbindgen_init_externref_table = function() {
            const table = wasm.__wbindgen_export_5;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
            ;
        };
        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        };
        imports.wbg.__wbindgen_is_object = function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        };
        imports.wbg.__wbindgen_is_string = function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        };
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        };
        imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        };
        imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        };
        imports.wbg.__wbindgen_link_9579f016b4522a24 = function(arg0) {
            const val = `onmessage = function (ev) {
                let [ia, index, value] = ev.data;
                ia = new Int32Array(ia.buffer);
                let result = Atomics.wait(ia, index, value);
                postMessage(result);
            };
            `;
            const ret = typeof URL.createObjectURL === 'undefined' ? "data:application/javascript," + encodeURIComponent(val) : URL.createObjectURL(new Blob([val], { type: "text/javascript" }));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return ret;
        };
        imports.wbg.__wbindgen_module = function() {
            const ret = __wbg_init.__wbindgen_wasm_module;
            return ret;
        };
        imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        };
        imports.wbg.__wbindgen_number_new = function(arg0) {
            const ret = arg0;
            return ret;
        };
        imports.wbg.__wbindgen_rethrow = function(arg0) {
            throw arg0;
        };
        imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };

        return imports;
    }

    function __wbg_init_memory(imports, memory) {
        imports.wbg.memory = memory || new WebAssembly.Memory({initial:18,maximum:16384,shared:true});
    }

    function __wbg_finalize_init(instance, module, thread_stack_size) {
        wasm = instance.exports;
        __wbg_init.__wbindgen_wasm_module = module;
        cachedDataViewMemory0 = null;
        cachedUint8ArrayMemory0 = null;

        if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) { throw 'invalid stack size' }
        wasm.__wbindgen_start(thread_stack_size);
        return wasm;
    }

    function initSync(module, memory) {
        if (wasm !== undefined) return wasm;

        let thread_stack_size
        if (typeof module !== 'undefined') {
            if (Object.getPrototypeOf(module) === Object.prototype) {
                ({module, memory, thread_stack_size} = module)
            } else {
                console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
            }
        }

        const imports = __wbg_get_imports();

        __wbg_init_memory(imports, memory);

        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }

        const instance = new WebAssembly.Instance(module, imports);

        return __wbg_finalize_init(instance, module, thread_stack_size);
    }

    async function __wbg_init(module_or_path, memory) {
        if (wasm !== undefined) return wasm;

        let thread_stack_size
        if (typeof module_or_path !== 'undefined') {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path, memory, thread_stack_size} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }

        if (typeof module_or_path === 'undefined' && typeof script_src !== 'undefined') {
            module_or_path = script_src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        __wbg_init_memory(imports, memory);

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module, thread_stack_size);
    }

    wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);

})();
