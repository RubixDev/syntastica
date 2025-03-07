use std::env;

const EXPORTED_FUNCTIONS: &[&str] = &[
    // custom functions
    "_add_language",
    "_highlight",
    "_process",
    "_render",
    "_get_builtin_theme",
    // stdlib symbols, see https://github.com/tree-sitter/tree-sitter/blob/v0.25.3/lib/src/wasm/stdlib-symbols.txt
    "_calloc",
    "_free",
    "_iswalnum",
    "_iswalpha",
    "_iswblank",
    "_iswdigit",
    "_iswlower",
    "_iswspace",
    "_iswupper",
    "_iswxdigit",
    "_malloc",
    "_memchr",
    "_memcmp",
    "_memcpy",
    "_memmove",
    "_memset",
    "_realloc",
    "_strcmp",
    "_strlen",
    "_strncat",
    "_strncmp",
    "_strncpy",
    "_towlower",
    "_towupper",
];

const EMSCRIPTEN_FLAGS: &[&str] = &[
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
        println!(
            "cargo::rustc-link-arg=-sEXPORTED_FUNCTIONS={}",
            EXPORTED_FUNCTIONS.join(",")
        );
        for flag in EMSCRIPTEN_FLAGS {
            println!("cargo::rustc-link-arg={flag}");
        }
    }
}
