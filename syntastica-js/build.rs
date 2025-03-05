use std::env;

const EMSCRIPTEN_FLAGS: &[&str] = &[
    "-sEXPORTED_FUNCTIONS=_add_language,_highlight,_process,_render,_get_builtin_theme,_free,_malloc",
    "-sEXPORT_ES6=1",
    "-sMODULARIZE=1",
    "-sEXPORTED_RUNTIME_METHODS=stringToNewUTF8,UTF8ToString,getValue,loadWebAssemblyModule",
    "-sINITIAL_MEMORY=33554432",
    "-sALLOW_MEMORY_GROWTH=1",
    "-sSUPPORT_BIG_ENDIAN=1",
    "-sMAIN_MODULE=2",
    "-sFILESYSTEM=0",
    "-sNODEJS_CATCH_EXIT=0",
    "-sNODEJS_CATCH_REJECTION=0",
];

fn main() {
    if env::var("TARGET").is_ok_and(|s| s == "wasm32-unknown-emscripten") {
        for flag in EMSCRIPTEN_FLAGS {
            println!("cargo::rustc-link-arg={flag}");
        }
    }
}
