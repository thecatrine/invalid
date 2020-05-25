/* tslint:disable */
/* eslint-disable */
/**
* @returns {number} 
*/
export function get_output_buffer_pointer(): number;
/**
* @param {number} px 
* @param {number} py 
* @param {number} angle 
* @param {number} angle_z 
*/
export function generate_board(px: number, py: number, angle: number, angle_z: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_output_buffer_pointer: () => number;
  readonly generate_board: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        