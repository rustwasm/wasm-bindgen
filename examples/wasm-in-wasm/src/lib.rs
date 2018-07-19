#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Like with the `dom` example this block will eventually be auto-generated, but
// for now we can write it by hand to get by!
#[wasm_bindgen]
extern "C" {
    type Module;
    #[wasm_bindgen(constructor, js_namespace = WebAssembly)]
    fn new(data: &[u8]) -> Module;

    type Instance;
    #[wasm_bindgen(constructor, js_namespace = WebAssembly)]
    fn new(module: Module) -> Instance;
    #[wasm_bindgen(method, getter, js_namespace = WebAssembly)]
    fn exports(this: &Instance) -> Exports;

    type Exports;
    #[wasm_bindgen(method, structural)]
    fn add(this: &Exports, a: u32, b: u32) -> u32;
    #[wasm_bindgen(method, getter, structural)]
    fn memory(this: &Exports) -> Memory;

    type Memory;
    #[wasm_bindgen(method, js_namespace = WebAssembly)]
    fn grow(this: &Memory, amt: u32) -> u32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("add.wasm");

#[wasm_bindgen]
pub fn run() {
    println!("instantiating a new wasm module directly");
    let a = Module::new(WASM);
    let b = Instance::new(a);
    let c = b.exports();

    println!("1 + 2 = {}", c.add(1, 2));
    let mem = c.memory();
    println!("created module has {} pages of memory", mem.grow(0));
    println!("giving the module 4 more pages of memory");
    mem.grow(4);
    println!("now the module has {} pages of memory", mem.grow(0));
}
