let wasm;

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_0.set(idx, obj);
    return idx;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_0.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
 * Takes a TM machine (.tm) code and turns it into a .dot graph code.
 *
 * # Examples
 *
 * In Rust, it would be used like this:
 * ```rust
 * println!("{}", parsing::tm_string_to_dot(&str, "Function Name", 1))
 * ```
 *
 * However, using it in an HTML context might be more understandable since this is
 * its main purpose.
 * ```html
 * <html>
 *   <head></head>
 *   <body>
 *     <textarea id="test-tm-code" name="test-tm-code" rows="20" cols="50">
 *       Turing Machine code goes here
 *     </textarea>
 *     <p id="test-dot-output">
 *       output
 *     </p>
 *     <script type="module">
 *       import init, { tm_string_to_dot } from "./tm_parser/tm_parser.js";
 *
 *       init().then(() => {
 *         const code_editor = document.getElementById("test-tm-code");
 *         const dot_output = document.getElementById("test-dot-output");
 *
 *         code_editor.addEventListener("input", (e) => {
 *           console.log("RUST CALL");
 *           dot_output.innerText = tm_string_to_dot(code_editor.value, "TEST");
 *           console.log("PARSED");
 *         });
 *       });
 *     </script>
 *   </body>
 * </html>
 * ```
 * @param {string} input_string
 * @param {string} tm_name
 * @param {number} grammar_version
 * @returns {string}
 */
export function tm_string_to_dot(input_string, tm_name, grammar_version) {
    let deferred4_0;
    let deferred4_1;
    try {
        const ptr0 = passStringToWasm0(input_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(tm_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.tm_string_to_dot(ptr0, len0, ptr1, len1, grammar_version);
        var ptr3 = ret[0];
        var len3 = ret[1];
        if (ret[3]) {
            ptr3 = 0; len3 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred4_0 = ptr3;
        deferred4_1 = len3;
        return getStringFromWasm0(ptr3, len3);
    } finally {
        wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
    }
}

/**
 * Enum to easily differentiate tape type.
 * @enum {0 | 1}
 */
export const TapeType = Object.freeze({
    /**
     * Work tape
     */
    Work: 0, "0": "Work",
    /**
     * Main tape
     */
    Main: 1, "1": "Main",
});

const SimuFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_simu_free(ptr >>> 0, 1));
/**
 * Turing Machine simulation object. This object is made to be usable
 * from JavaScript in the web via WebAssembly.
 */
export class Simu {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Simu.prototype);
        obj.__wbg_ptr = ptr;
        SimuFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SimuFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simu_free(ptr, 0);
    }
    /**
     * Simulation object constructor.
     *
     * After this call, the returned object can directly be used
     * to run the parsed machine.
     * @param {string} input_string
     * @param {number} grammar_version
     * @param {Uint8Array} main_tape
     * @param {Uint8Array} work_tape
     * @param {string[]} fun_env
     * @returns {Simu}
     */
    static new(input_string, grammar_version, main_tape, work_tape, fun_env) {
        const ptr0 = passStringToWasm0(input_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(main_tape, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(work_tape, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArrayJsValueToWasm0(fun_env, wasm.__wbindgen_malloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.simu_new(ptr0, len0, grammar_version, ptr1, len1, ptr2, len2, ptr3, len3);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Simu.__wrap(ret[0]);
    }
    /**
     * Checks if the Simulation's TM has not started yet.
     * NB: this is NOT a check for the START state.
     * @returns {boolean}
     */
    is_start() {
        const ret = wasm.simu_is_start(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Checks if the Simulation's TM has reached the END state.
     * @returns {boolean}
     */
    is_end() {
        const ret = wasm.simu_is_end(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Checks if the Simulation's TM has reached the ERROR state.
     * @returns {boolean}
     */
    is_error() {
        const ret = wasm.simu_is_error(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Runs a single step (i.e. takes a single transition) of the
     * simulated Turing Machine.
     */
    next_step() {
        wasm.simu_next_step(this.__wbg_ptr);
    }
    /**
     * Rewinds the Turing Machine one step back.
     */
    prev_step() {
        wasm.simu_prev_step(this.__wbg_ptr);
    }
    /**
     * Runs the whole Turing Machine for a maxiumum number of iterations.
     *
     * We simply call `_next_step` in a while not finished loop.
     */
    all_steps() {
        wasm.simu_all_steps(this.__wbg_ptr);
    }
    /**
     * Exposed function that resets the simulation
     * - sets the current state back to START
     * - sets the tapes to the given values
     * - sets the head positions to 0
     * @param {Uint8Array} main_tape
     * @param {Uint8Array} work_tape
     */
    reset(main_tape, work_tape) {
        const ptr0 = passArray8ToWasm0(main_tape, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(work_tape, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.simu_reset(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * Verifies that the current main tape has the expected result given
     * in arguments.
     *
     * Returns true if on END state and the expected output matches the main tape.
     * @param {Uint8Array} expected
     * @returns {boolean}
     */
    verify_output(expected) {
        const ptr0 = passArray8ToWasm0(expected, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.simu_verify_output(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Returns the current state ID of the simulated Turing Machine.
     * @returns {string}
     */
    get_current_state() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.simu_get_current_state(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Returns the current position of the main tape's head of the simulated TM.
     * @returns {number}
     */
    head_pos_main() {
        const ret = wasm.simu_head_pos_main(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Returns the current position of the work tape's head of the simulated TM.
     * @returns {number}
     */
    head_pos_work() {
        const ret = wasm.simu_head_pos_work(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Returns the main tape
     * @returns {Uint8Array}
     */
    get_main_tape() {
        const ret = wasm.simu_get_main_tape(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Returns the work tape
     * @returns {Uint8Array}
     */
    get_work_tape() {
        const ret = wasm.simu_get_work_tape(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}

const TapeEditFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tapeedit_free(ptr >>> 0, 1));
/**
 * Struct containing an edition of a tape cell.
 */
export class TapeEdit {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TapeEditFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tapeedit_free(ptr, 0);
    }
}

const TmEditFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tmedit_free(ptr >>> 0, 1));
/**
 * Struct used to store Tape edits
 */
export class TmEdit {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TmEditFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tmedit_free(ptr, 0);
    }
}

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
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
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

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('tm_parser_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
