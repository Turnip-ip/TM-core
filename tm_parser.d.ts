/* tslint:disable */
/* eslint-disable */
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
export function tm_string_to_dot(input_string: string, tm_name: string, grammar_version: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly tm_string_to_dot: (a: number, b: number, c: number, d: number, e: number) => Array;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
