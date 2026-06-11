#![no_std]
#![warn(rust_2018_idioms)]

use tree_sitter::Language;

syntastica_macros::js_lang_info!();
syntastica_macros::js_lang_lib!("rush");
