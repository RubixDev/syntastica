use std::env;

const EMSCRIPTEN_FLAGS: &[&str] = &[
    "-sEXPORTED_FUNCTIONS=_init,_highlight,_process,_render,_free,_malloc",
    "-sEXPORT_ES6=1",
    "-sMODULARIZE=1",
    "-sEXPORTED_RUNTIME_METHODS=stringToNewUTF8,UTF8ToString,getValue,setValue",
];

fn main() {
    if env::var("TARGET").map_or(false, |s| s == "wasm32-unknown-emscripten") {
        for flag in EMSCRIPTEN_FLAGS {
            println!("cargo:rustc-link-arg={flag}");
        }
    }
}
