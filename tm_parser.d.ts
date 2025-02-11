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
 */
export function tm_string_to_dot(input_string: string, tm_name: string, grammar_version: number): string;
/**
 * Enum to easily differentiate tape type.
 */
export enum TapeType {
  /**
   * Work tape
   */
  Work = 0,
  /**
   * Main tape
   */
  Main = 1,
}
/**
 * Turing Machine simulation object. This object is made to be usable
 * from JavaScript in the web via WebAssembly.
 */
export class Simu {
  private constructor();
  free(): void;
  /**
   * Simulation object constructor.
   *
   * After this call, the returned object can directly be used
   * to run the parsed machine.
   */
  static new(input_string: string, grammar_version: number, main_tape: Uint8Array, work_tape: Uint8Array, fun_env: string[]): Simu;
  /**
   * Checks if the Simulation's TM has not started yet.
   * NB: this is NOT a check for the START state.
   */
  is_start(): boolean;
  /**
   * Checks if the Simulation's TM has reached the END state.
   */
  is_end(): boolean;
  /**
   * Checks if the Simulation's TM has reached the ERROR state.
   */
  is_error(): boolean;
  /**
   * TODO: documentation
   */
  static edit_main_tape(): void;
  /**
   * TODO: documentation
   */
  static edit_work_tape(): void;
  /**
   * Verifies that the current main tape has the expected result given
   * in arguments.
   *
   * Returns true if on END state and the expected output matches the main tape.
   */
  verify_output(expected: Uint8Array): boolean;
  /**
   * Returns the current state ID of the simulated Turing Machine.
   */
  get_current_state(): string;
  /**
   * Returns the current position of the main tape's head of the simulated TM.
   */
  head_pos_main(): number;
  /**
   * Returns the current position of the work tape's head of the simulated TM.
   */
  head_pos_work(): number;
  /**
   * Returns the main tape
   */
  get_main_tape(): Uint8Array;
  /**
   * Returns the work tape
   */
  get_work_tape(): Uint8Array;
}
/**
 * Struct containing an edition of a tape cell.
 */
export class TapeEdit {
  private constructor();
  free(): void;
}
/**
 * Struct used to store Tape edits
 */
export class TmEdit {
  private constructor();
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_tapeedit_free: (a: number, b: number) => void;
  readonly __wbg_tmedit_free: (a: number, b: number) => void;
  readonly __wbg_simu_free: (a: number, b: number) => void;
  readonly simu_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number, number];
  readonly simu_is_start: (a: number) => number;
  readonly simu_is_end: (a: number) => number;
  readonly simu_is_error: (a: number) => number;
  readonly simu_edit_main_tape: () => void;
  readonly simu_edit_work_tape: () => void;
  readonly simu_verify_output: (a: number, b: number, c: number) => number;
  readonly simu_get_current_state: (a: number) => [number, number];
  readonly simu_head_pos_main: (a: number) => number;
  readonly simu_head_pos_work: (a: number) => number;
  readonly simu_get_main_tape: (a: number) => [number, number];
  readonly simu_get_work_tape: (a: number) => [number, number];
  readonly tm_string_to_dot: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
