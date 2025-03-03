const EMSCRIPTEN_FLAGS: &[&str] = &[
    "-sTOTAL_MEMORY=33554432",
    "-sSIDE_MODULE=2",
    "-sWASM=1",
    "-sNODEJS_CATCH_EXIT=0",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("TARGET").is_ok_and(|s| s == "wasm32-unknown-emscripten") {
        for flag in EMSCRIPTEN_FLAGS {
            println!("cargo::rustc-link-arg={flag}");
        }

        syntastica_macros::js_lang_build!("json");
    }
    Ok(())
}

include!("../../../syntastica-parsers-git/build_helper.rs");
