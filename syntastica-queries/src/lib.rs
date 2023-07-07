#![doc = include_str!("../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub const ASM_HIGHLIGHTS: &str = include_str!("../generated_queries/asm/highlights.scm");
pub const ASM_INJECTIONS: &str = include_str!("../generated_queries/asm/injections.scm");
pub const ASM_LOCALS: &str = include_str!("../generated_queries/asm/locals.scm");
pub const ASM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/asm/highlights_crates_io.scm");
pub const ASM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/asm/injections_crates_io.scm");
pub const ASM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/asm/locals_crates_io.scm");

pub const BASH_HIGHLIGHTS: &str = include_str!("../generated_queries/bash/highlights.scm");
pub const BASH_INJECTIONS: &str = include_str!("../generated_queries/bash/injections.scm");
pub const BASH_LOCALS: &str = include_str!("../generated_queries/bash/locals.scm");
pub const BASH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bash/highlights_crates_io.scm");
pub const BASH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bash/injections_crates_io.scm");
pub const BASH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bash/locals_crates_io.scm");

pub const C_HIGHLIGHTS: &str = include_str!("../generated_queries/c/highlights.scm");
pub const C_INJECTIONS: &str = include_str!("../generated_queries/c/injections.scm");
pub const C_LOCALS: &str = include_str!("../generated_queries/c/locals.scm");
pub const C_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/c/highlights_crates_io.scm");
pub const C_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/c/injections_crates_io.scm");
pub const C_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/c/locals_crates_io.scm");

pub const C_SHARP_HIGHLIGHTS: &str = include_str!("../generated_queries/c_sharp/highlights.scm");
pub const C_SHARP_INJECTIONS: &str = include_str!("../generated_queries/c_sharp/injections.scm");
pub const C_SHARP_LOCALS: &str = include_str!("../generated_queries/c_sharp/locals.scm");
pub const C_SHARP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/highlights_crates_io.scm");
pub const C_SHARP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/injections_crates_io.scm");
pub const C_SHARP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/locals_crates_io.scm");

pub const CPP_HIGHLIGHTS: &str = include_str!("../generated_queries/cpp/highlights.scm");
pub const CPP_INJECTIONS: &str = include_str!("../generated_queries/cpp/injections.scm");
pub const CPP_LOCALS: &str = include_str!("../generated_queries/cpp/locals.scm");
pub const CPP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cpp/highlights_crates_io.scm");
pub const CPP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cpp/injections_crates_io.scm");
pub const CPP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cpp/locals_crates_io.scm");

pub const CSS_HIGHLIGHTS: &str = include_str!("../generated_queries/css/highlights.scm");
pub const CSS_INJECTIONS: &str = include_str!("../generated_queries/css/injections.scm");
pub const CSS_LOCALS: &str = include_str!("../generated_queries/css/locals.scm");
pub const CSS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/css/highlights_crates_io.scm");
pub const CSS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/css/injections_crates_io.scm");
pub const CSS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/css/locals_crates_io.scm");

pub const EJS_HIGHLIGHTS: &str = include_str!("../generated_queries/ejs/highlights.scm");
pub const EJS_INJECTIONS: &str = include_str!("../generated_queries/ejs/injections.scm");
pub const EJS_LOCALS: &str = include_str!("../generated_queries/ejs/locals.scm");
pub const EJS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ejs/highlights_crates_io.scm");
pub const EJS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ejs/injections_crates_io.scm");
pub const EJS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ejs/locals_crates_io.scm");

pub const ERB_HIGHLIGHTS: &str = include_str!("../generated_queries/erb/highlights.scm");
pub const ERB_INJECTIONS: &str = include_str!("../generated_queries/erb/injections.scm");
pub const ERB_LOCALS: &str = include_str!("../generated_queries/erb/locals.scm");
pub const ERB_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/erb/highlights_crates_io.scm");
pub const ERB_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/erb/injections_crates_io.scm");
pub const ERB_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/erb/locals_crates_io.scm");

pub const GO_HIGHLIGHTS: &str = include_str!("../generated_queries/go/highlights.scm");
pub const GO_INJECTIONS: &str = include_str!("../generated_queries/go/injections.scm");
pub const GO_LOCALS: &str = include_str!("../generated_queries/go/locals.scm");
pub const GO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/go/highlights_crates_io.scm");
pub const GO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/go/injections_crates_io.scm");
pub const GO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/go/locals_crates_io.scm");

pub const HASKELL_HIGHLIGHTS: &str = include_str!("../generated_queries/haskell/highlights.scm");
pub const HASKELL_INJECTIONS: &str = include_str!("../generated_queries/haskell/injections.scm");
pub const HASKELL_LOCALS: &str = include_str!("../generated_queries/haskell/locals.scm");
pub const HASKELL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/haskell/highlights_crates_io.scm");
pub const HASKELL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/haskell/injections_crates_io.scm");
pub const HASKELL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/haskell/locals_crates_io.scm");

pub const HEXDUMP_HIGHLIGHTS: &str = include_str!("../generated_queries/hexdump/highlights.scm");
pub const HEXDUMP_INJECTIONS: &str = include_str!("../generated_queries/hexdump/injections.scm");
pub const HEXDUMP_LOCALS: &str = include_str!("../generated_queries/hexdump/locals.scm");
pub const HEXDUMP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/highlights_crates_io.scm");
pub const HEXDUMP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/injections_crates_io.scm");
pub const HEXDUMP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/locals_crates_io.scm");

pub const HTML_HIGHLIGHTS: &str = include_str!("../generated_queries/html/highlights.scm");
pub const HTML_INJECTIONS: &str = include_str!("../generated_queries/html/injections.scm");
pub const HTML_LOCALS: &str = include_str!("../generated_queries/html/locals.scm");
pub const HTML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/html/highlights_crates_io.scm");
pub const HTML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/html/injections_crates_io.scm");
pub const HTML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/html/locals_crates_io.scm");

pub const JAVA_HIGHLIGHTS: &str = include_str!("../generated_queries/java/highlights.scm");
pub const JAVA_INJECTIONS: &str = include_str!("../generated_queries/java/injections.scm");
pub const JAVA_LOCALS: &str = include_str!("../generated_queries/java/locals.scm");
pub const JAVA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/java/highlights_crates_io.scm");
pub const JAVA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/java/injections_crates_io.scm");
pub const JAVA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/java/locals_crates_io.scm");

pub const JAVASCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/javascript/highlights.scm");
pub const JAVASCRIPT_INJECTIONS: &str = include_str!("../generated_queries/javascript/injections.scm");
pub const JAVASCRIPT_LOCALS: &str = include_str!("../generated_queries/javascript/locals.scm");
pub const JAVASCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/javascript/highlights_crates_io.scm");
pub const JAVASCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/javascript/injections_crates_io.scm");
pub const JAVASCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/javascript/locals_crates_io.scm");

pub const JSDOC_HIGHLIGHTS: &str = include_str!("../generated_queries/jsdoc/highlights.scm");
pub const JSDOC_INJECTIONS: &str = include_str!("../generated_queries/jsdoc/injections.scm");
pub const JSDOC_LOCALS: &str = include_str!("../generated_queries/jsdoc/locals.scm");
pub const JSDOC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/highlights_crates_io.scm");
pub const JSDOC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/injections_crates_io.scm");
pub const JSDOC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/locals_crates_io.scm");

pub const JSON_HIGHLIGHTS: &str = include_str!("../generated_queries/json/highlights.scm");
pub const JSON_INJECTIONS: &str = include_str!("../generated_queries/json/injections.scm");
pub const JSON_LOCALS: &str = include_str!("../generated_queries/json/locals.scm");
pub const JSON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/json/highlights_crates_io.scm");
pub const JSON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/json/injections_crates_io.scm");
pub const JSON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/json/locals_crates_io.scm");

pub const OCAML_HIGHLIGHTS: &str = include_str!("../generated_queries/ocaml/highlights.scm");
pub const OCAML_INJECTIONS: &str = include_str!("../generated_queries/ocaml/injections.scm");
pub const OCAML_LOCALS: &str = include_str!("../generated_queries/ocaml/locals.scm");
pub const OCAML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/highlights_crates_io.scm");
pub const OCAML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/injections_crates_io.scm");
pub const OCAML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/locals_crates_io.scm");

pub const OCAML_INTERFACE_HIGHLIGHTS: &str = include_str!("../generated_queries/ocaml_interface/highlights.scm");
pub const OCAML_INTERFACE_INJECTIONS: &str = include_str!("../generated_queries/ocaml_interface/injections.scm");
pub const OCAML_INTERFACE_LOCALS: &str = include_str!("../generated_queries/ocaml_interface/locals.scm");
pub const OCAML_INTERFACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/highlights_crates_io.scm");
pub const OCAML_INTERFACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/injections_crates_io.scm");
pub const OCAML_INTERFACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/locals_crates_io.scm");

pub const PHP_HIGHLIGHTS: &str = include_str!("../generated_queries/php/highlights.scm");
pub const PHP_INJECTIONS: &str = include_str!("../generated_queries/php/injections.scm");
pub const PHP_LOCALS: &str = include_str!("../generated_queries/php/locals.scm");
pub const PHP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/php/highlights_crates_io.scm");
pub const PHP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/php/injections_crates_io.scm");
pub const PHP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/php/locals_crates_io.scm");

pub const PYTHON_HIGHLIGHTS: &str = include_str!("../generated_queries/python/highlights.scm");
pub const PYTHON_INJECTIONS: &str = include_str!("../generated_queries/python/injections.scm");
pub const PYTHON_LOCALS: &str = include_str!("../generated_queries/python/locals.scm");
pub const PYTHON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/python/highlights_crates_io.scm");
pub const PYTHON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/python/injections_crates_io.scm");
pub const PYTHON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/python/locals_crates_io.scm");

pub const QL_HIGHLIGHTS: &str = include_str!("../generated_queries/ql/highlights.scm");
pub const QL_INJECTIONS: &str = include_str!("../generated_queries/ql/injections.scm");
pub const QL_LOCALS: &str = include_str!("../generated_queries/ql/locals.scm");
pub const QL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ql/highlights_crates_io.scm");
pub const QL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ql/injections_crates_io.scm");
pub const QL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ql/locals_crates_io.scm");

pub const REGEX_HIGHLIGHTS: &str = include_str!("../generated_queries/regex/highlights.scm");
pub const REGEX_INJECTIONS: &str = include_str!("../generated_queries/regex/injections.scm");
pub const REGEX_LOCALS: &str = include_str!("../generated_queries/regex/locals.scm");
pub const REGEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/regex/highlights_crates_io.scm");
pub const REGEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/regex/injections_crates_io.scm");
pub const REGEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/regex/locals_crates_io.scm");

pub const RUBY_HIGHLIGHTS: &str = include_str!("../generated_queries/ruby/highlights.scm");
pub const RUBY_INJECTIONS: &str = include_str!("../generated_queries/ruby/injections.scm");
pub const RUBY_LOCALS: &str = include_str!("../generated_queries/ruby/locals.scm");
pub const RUBY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ruby/highlights_crates_io.scm");
pub const RUBY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ruby/injections_crates_io.scm");
pub const RUBY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ruby/locals_crates_io.scm");

pub const RUSH_HIGHLIGHTS: &str = include_str!("../generated_queries/rush/highlights.scm");
pub const RUSH_INJECTIONS: &str = include_str!("../generated_queries/rush/injections.scm");
pub const RUSH_LOCALS: &str = include_str!("../generated_queries/rush/locals.scm");
pub const RUSH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rush/highlights_crates_io.scm");
pub const RUSH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rush/injections_crates_io.scm");
pub const RUSH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rush/locals_crates_io.scm");

pub const RUST_HIGHLIGHTS: &str = include_str!("../generated_queries/rust/highlights.scm");
pub const RUST_INJECTIONS: &str = include_str!("../generated_queries/rust/injections.scm");
pub const RUST_LOCALS: &str = include_str!("../generated_queries/rust/locals.scm");
pub const RUST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rust/highlights_crates_io.scm");
pub const RUST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rust/injections_crates_io.scm");
pub const RUST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rust/locals_crates_io.scm");

pub const SCALA_HIGHLIGHTS: &str = include_str!("../generated_queries/scala/highlights.scm");
pub const SCALA_INJECTIONS: &str = include_str!("../generated_queries/scala/injections.scm");
pub const SCALA_LOCALS: &str = include_str!("../generated_queries/scala/locals.scm");
pub const SCALA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/scala/highlights_crates_io.scm");
pub const SCALA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/scala/injections_crates_io.scm");
pub const SCALA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/scala/locals_crates_io.scm");

pub const TSX_HIGHLIGHTS: &str = include_str!("../generated_queries/tsx/highlights.scm");
pub const TSX_INJECTIONS: &str = include_str!("../generated_queries/tsx/injections.scm");
pub const TSX_LOCALS: &str = include_str!("../generated_queries/tsx/locals.scm");
pub const TSX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tsx/highlights_crates_io.scm");
pub const TSX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tsx/injections_crates_io.scm");
pub const TSX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tsx/locals_crates_io.scm");

pub const TYPESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/typescript/highlights.scm");
pub const TYPESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/typescript/injections.scm");
pub const TYPESCRIPT_LOCALS: &str = include_str!("../generated_queries/typescript/locals.scm");
pub const TYPESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/typescript/highlights_crates_io.scm");
pub const TYPESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/typescript/injections_crates_io.scm");
pub const TYPESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/typescript/locals_crates_io.scm");

pub const VERILOG_HIGHLIGHTS: &str = include_str!("../generated_queries/verilog/highlights.scm");
pub const VERILOG_INJECTIONS: &str = include_str!("../generated_queries/verilog/injections.scm");
pub const VERILOG_LOCALS: &str = include_str!("../generated_queries/verilog/locals.scm");
pub const VERILOG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/verilog/highlights_crates_io.scm");
pub const VERILOG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/verilog/injections_crates_io.scm");
pub const VERILOG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/verilog/locals_crates_io.scm");

pub const WAT_HIGHLIGHTS: &str = include_str!("../generated_queries/wat/highlights.scm");
pub const WAT_INJECTIONS: &str = include_str!("../generated_queries/wat/injections.scm");
pub const WAT_LOCALS: &str = include_str!("../generated_queries/wat/locals.scm");
pub const WAT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wat/highlights_crates_io.scm");
pub const WAT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wat/injections_crates_io.scm");
pub const WAT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wat/locals_crates_io.scm");
