/* tslint:disable */
/**
*/
export function init_storage(): void;
/**
* @param {string} input 
*/
export function render_all(input: string): void;
/**
* @param {string} input 
*/
export function render_successes(input: string): void;
/**
* @param {string} input 
*/
export function render_failures(input: string): void;
/**
* @param {string} id 
* @param {string} value 
*/
export function change_value(id: string, value: string): void;
/**
* @param {string} id 
* @returns {string} 
*/
export function get_value(id: string): string;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        