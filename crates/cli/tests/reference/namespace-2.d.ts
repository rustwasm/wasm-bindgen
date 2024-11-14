declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	 * C-style enum
	 */
	export enum ImageFormat {
	  PNG = 0,
	  JPEG = 1,
	  GIF = 2,
	}
	/**
	 * String enum
	 */
	type Status = "success" | "failure";
	export namespace ImageFormat {
	  export function from_str(s: string): ImageFormat;
	}
	export namespace Status {
	  /**
	   * I have documentation.
	   */
	  export function from_bool(success: boolean): Status;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly imageformat_from_str: (a: number, b: number) => number;
  readonly imageformat_is_lossless: (a: number) => number;
  readonly status_from_bool: (a: number) => number;
  readonly status_to_bool: (a: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
