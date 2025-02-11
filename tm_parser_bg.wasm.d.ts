/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_tapeedit_free: (a: number, b: number) => void;
export const __wbg_tmedit_free: (a: number, b: number) => void;
export const __wbg_simu_free: (a: number, b: number) => void;
export const simu_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number, number];
export const simu_is_start: (a: number) => number;
export const simu_is_end: (a: number) => number;
export const simu_is_error: (a: number) => number;
export const simu_edit_main_tape: () => void;
export const simu_edit_work_tape: () => void;
export const simu_verify_output: (a: number, b: number, c: number) => number;
export const simu_get_current_state: (a: number) => [number, number];
export const simu_head_pos_main: (a: number) => number;
export const simu_head_pos_work: (a: number) => number;
export const simu_get_main_tape: (a: number) => [number, number];
export const simu_get_work_tape: (a: number) => [number, number];
export const tm_string_to_dot: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
export const __wbindgen_export_0: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __externref_table_alloc: () => number;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
