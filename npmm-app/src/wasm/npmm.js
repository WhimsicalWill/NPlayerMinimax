import wasm from "./npmm_bg.wasm";
import { __wbg_set_wasm } from "./npmm_bg.js";
__wbg_set_wasm(wasm);
export * from "./npmm_bg.js";

wasm.__wbindgen_start();
