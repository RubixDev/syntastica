macro_rules! langs {
    ($($feat:literal, $name:literal, $url:literal, $branch:literal, external_c = $c:tt, external_cpp = $cpp:tt $(, $path:literal)?);* $(;)?) => {$(
        #[cfg(feature = $feat)]
        {
            // clone repo into `parsers/{name}`, if it does not already exists
            let repo_dir = std::path::Path::new(concat!("parsers/", $name));
            if !repo_dir.exists() {
                println!("cloning repository for {}", $name);
                std::process::Command::new("git")
                    .args([
                        "clone",
                        $url,
                        "--depth=1",
                        "--branch",
                        $branch,
                        "--single-branch"
                    ])
                    .arg(repo_dir)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()?;
            }

            let src_dir = repo_dir$(.join($path))?.join("src");

            println!("building parser for {}", $name);
            let mut c_config = cc::Build::new();
            c_config.include(&src_dir);
            c_config
                .flag_if_supported("-Wno-unused-parameter")
                .flag_if_supported("-Wno-unused-but-set-variable")
                .flag_if_supported("-Wno-trigraphs")
                .flag_if_supported("-w");
            let parser_path = src_dir.join("parser.c");
            c_config.file(&parser_path);

            langs!(@c $c, src_dir, c_config);

            println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
            c_config.compile(&concat!("parser_", $name $(, $path)?).replace('/', "_"));
            println!("finished building parser for {}", $name);

            langs!(@cpp $cpp, src_dir, $name, $($path)?);
        }
    )*};
    (@c true, $src_dir:ident, $c_config:ident) => {
        let scanner_path = $src_dir.join("scanner.c");
        $c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    };
    (@c false, $src_dir:ident, $c_config:ident) => {};
    (@cpp true, $src_dir:ident, $name:literal, $($path:literal)?) => {
        println!("building cpp scanner for {}", $name);
        let mut cpp_config = cc::Build::new();
        cpp_config.cpp(true);
        cpp_config.include(&$src_dir);
        cpp_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-w");
        let scanner_path = $src_dir.join("scanner.cc");
        cpp_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
        cpp_config.compile(&concat!("scanner_", $name $(, $path)?).replace('/', "_"));
        println!("finished building cpp scanner for {}", $name);
    };
    (@cpp false, $src_dir:ident, $name:literal, $($path:literal)?) => {};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    langs! {
        "some", "bash", "https://github.com/tree-sitter/tree-sitter-bash", "master", external_c = false, external_cpp = true;
        "some", "c", "https://github.com/tree-sitter/tree-sitter-c", "master", external_c = false, external_cpp = false;
        "some", "cpp", "https://github.com/tree-sitter/tree-sitter-cpp", "master", external_c = false, external_cpp = true;
        "some", "css", "https://github.com/tree-sitter/tree-sitter-css", "master", external_c = true, external_cpp = false;
        "some", "go", "https://github.com/tree-sitter/tree-sitter-go", "master", external_c = false, external_cpp = false;
        "some", "html", "https://github.com/tree-sitter/tree-sitter-html", "master", external_c = false, external_cpp = true;
        "some", "java", "https://github.com/tree-sitter/tree-sitter-java", "master", external_c = false, external_cpp = false;
        "some", "javascript", "https://github.com/tree-sitter/tree-sitter-javascript", "master", external_c = true, external_cpp = false;
        "some", "json", "https://github.com/tree-sitter/tree-sitter-json", "master", external_c = false, external_cpp = false;
        "some", "python", "https://github.com/tree-sitter/tree-sitter-python", "master", external_c = false, external_cpp = true;
        "some", "rust", "https://github.com/tree-sitter/tree-sitter-rust", "master", external_c = true, external_cpp = false;
        "some", "typescript", "https://github.com/tree-sitter/tree-sitter-typescript", "master", external_c = true, external_cpp = false, "tsx";
        "some", "typescript", "https://github.com/tree-sitter/tree-sitter-typescript", "master", external_c = true, external_cpp = false, "typescript";

        "most", "asm", "https://github.com/rush-rs/tree-sitter-asm", "main", external_c = false, external_cpp = false;
        "most", "c-sharp", "https://github.com/tree-sitter/tree-sitter-c-sharp", "master", external_c = true, external_cpp = false;
        "most", "haskell", "https://github.com/tree-sitter/tree-sitter-haskell", "master", external_c = true, external_cpp = false;
        "most", "jsdoc", "https://github.com/tree-sitter/tree-sitter-jsdoc", "master", external_c = false, external_cpp = false;
        "most", "php", "https://github.com/tree-sitter/tree-sitter-php", "master", external_c = false, external_cpp = true;
        "most", "regex", "https://github.com/tree-sitter/tree-sitter-regex", "master", external_c = false, external_cpp = false;
        "most", "ruby", "https://github.com/tree-sitter/tree-sitter-ruby", "master", external_c = false, external_cpp = true;
        "most", "scala", "https://github.com/tree-sitter/tree-sitter-scala", "master", external_c = true, external_cpp = false;

        "all", "embedded-template", "https://github.com/tree-sitter/tree-sitter-embedded-template", "master", external_c = false, external_cpp = false;
        "all", "hexdump", "https://github.com/rush-rs/tree-sitter-hexdump", "main", external_c = false, external_cpp = false;
        "all", "julia", "https://github.com/tree-sitter/tree-sitter-julia", "master", external_c = true, external_cpp = false;
        "all", "ocaml", "https://github.com/tree-sitter/tree-sitter-ocaml", "master", external_c = false, external_cpp = true, "ocaml";
        "all", "ocaml", "https://github.com/tree-sitter/tree-sitter-ocaml", "master", external_c = false, external_cpp = true, "interface";
        "all", "ql", "https://github.com/tree-sitter/tree-sitter-ql", "master", external_c = false, external_cpp = false;
        "all", "rush", "https://github.com/rush-rs/tree-sitter-rush", "main", external_c = false, external_cpp = false;
        "all", "verilog", "https://github.com/tree-sitter/tree-sitter-verilog", "master", external_c = false, external_cpp = false;
        "all", "wasm", "https://github.com/wasm-lsp/tree-sitter-wasm", "main", external_c = false, external_cpp = false, "wat";
        // "all", "wasm", "https://github.com/wasm-lsp/tree-sitter-wasm", "main", external_c = false, external_cpp = false, "wast";
    }
    Ok(())
}
