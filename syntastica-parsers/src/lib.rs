#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

macro_rules! langs {
    ($($feat:literal, $name:ident, $src:ident);* $(;)?) => {
        extern "C" {$(
            #[cfg(feature = $feat)]
            fn $src() -> tree_sitter::Language;
        )*}
        $(
            #[cfg(feature = $feat)]
            pub fn $name() -> tree_sitter::Language {
                unsafe { $src() }
            }
        )*
    };
}

langs! {
    "some", bash, tree_sitter_bash;
    "some", c, tree_sitter_c;
    "some", cpp, tree_sitter_cpp;
    "some", css, tree_sitter_css;
    "some", go, tree_sitter_go;
    "some", html, tree_sitter_html;
    "some", java, tree_sitter_java;
    "some", javascript, tree_sitter_javascript;
    "some", json, tree_sitter_json;
    "some", python, tree_sitter_python;
    "some", rust, tree_sitter_rust;
    "some", tsx, tree_sitter_tsx;
    "some", typescript, tree_sitter_typescript;

    "most", asm, tree_sitter_asm;
    "most", c_sharp, tree_sitter_c_sharp;
    "most", haskell, tree_sitter_haskell;
    "most", jsdoc, tree_sitter_jsdoc;
    "most", php, tree_sitter_php;
    "most", regex, tree_sitter_regex;
    "most", ruby, tree_sitter_ruby;
    "most", scala, tree_sitter_scala;

    "all", embedded_template, tree_sitter_embedded_template;
    "all", hexdump, tree_sitter_hexdump;
    "all", julia, tree_sitter_julia;
    "all", ocaml, tree_sitter_ocaml;
    "all", ocaml_interface, tree_sitter_ocaml_interface;
    "all", ql, tree_sitter_ql;
    "all", rush, tree_sitter_rush;
    "all", verilog, tree_sitter_verilog;
    "all", wat, tree_sitter_wat;

    // TODO: llvm
    // TODO: ebnf
}
