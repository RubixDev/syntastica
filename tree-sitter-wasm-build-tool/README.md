# `tree-sitter-wasm-build-tool`

A crate to easily allow tree-sitter parsers to compile to Rust's
`wasm32-unknown-unknown` target.

Currently, this only works with parsers that do _not_ make use of an external
C++ scanner.

It is meant to be used in build scripts, typically only if some
<span class="stab portability"><code>wasm</code></span> or
<span class="stab portability"><code>c2rust</code></span> feature is enabled.

For example:

```rust,no_run
let src_dir = std::path::Path::new("src");
let mut c_config = cc::Build::new();
c_config.include(src_dir);
c_config
    .flag("-Wno-unused-parameter")
    .flag("-Wno-unused-but-set-variable")
    .flag("-Wno-trigraphs");
let parser_path = src_dir.join("parser.c");
c_config.file(&parser_path);

#[cfg(feature = "c2rust")]
tree_sitter_wasm_build_tool::add_wasm_headers(&mut c_config).unwrap();

c_config.compile("parser");
println!("cargo::rerun-if-changed={}", parser_path.display());
```

The only public function is [`add_wasm_headers`]. See its documentation for some
more information.
