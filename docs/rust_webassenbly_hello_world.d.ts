/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @returns {string}
*/
export function greet(name: string): string;
/**
* @param {string} x
* @param {string} y
* @returns {string}
*/
export function gcd(x: string, y: string): string;
/**
* @param {number} x
* @param {number} y
* @returns {number}
*/
export function gcd_int(x: number, y: number): number;
/**
* @param {string} number
* @returns {string}
*/
export function fizzbuzz(number: string): string;
/**
* @param {number} number
* @returns {string}
*/
export function fizzbuzz_int(number: number): string;
/**
* @param {bigint} number
* @returns {string}
*/
export function fizzbuzz_bigint(number: bigint): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number, c: number) => void;
  readonly gcd: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly gcd_int: (a: number, b: number) => number;
  readonly fizzbuzz: (a: number, b: number, c: number) => void;
  readonly fizzbuzz_int: (a: number, b: number) => void;
  readonly fizzbuzz_bigint: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
