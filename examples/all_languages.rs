use std::collections::BTreeMap;

use syntastica::{
    providers::{ConfiguredLanguages, ParserProvider},
    renderer::TerminalRenderer,
    Highlighter,
};
use syntastica_parsers_git::ParserProviderGit;

pub const HTML_INJECTIONS: &str = "(\n  (style_element\n    (start_tag) @_no_type_lang\n    (#not-lua-match? @_no_type_lang \"%slang%s*=\")\n    (#not-lua-match? @_no_type_lang \"%stype%s*=\")\n    (raw_text) @injection.content\n    (#set! injection.language \"injection.content\")\n  )\n  (#set! injection.language \"css\")\n)\n\n(\n  (style_element\n    (start_tag\n      (attribute\n        (attribute_name) @_type\n        (quoted_attribute_value\n          (attribute_value) @_css\n        )\n      )\n    )\n    (raw_text) @injection.content\n    (#set! injection.language \"injection.content\")\n  )\n  (#eq? @_type \"type\")\n  (#eq? @_css \"text/css\")\n  (#set! injection.language \"css\")\n)\n\n(\n  (script_element\n    (start_tag) @_no_type_lang\n    (#not-lua-match? @_no_type_lang \"%slang%s*=\")\n    (#not-lua-match? @_no_type_lang \"%stype%s*=\")\n    (raw_text) @injection.content\n    (#set! injection.language \"injection.content\")\n  )\n  (#set! injection.language \"javascript\")\n)\n\n(script_element\n  (start_tag\n    (\n      (attribute\n        (attribute_name) @_attr\n        (#eq? @_attr \"type\")\n        (quoted_attribute_value\n          (attribute_value) @injection.content\n          (#set! injection.language \"injection.language\")\n        )\n      )\n    )\n  )\n  (raw_text) @injection.content\n  (#set! injection.language \"injection.content\")\n)\n\n(\n  (attribute\n    (attribute_name) @_attr\n    (quoted_attribute_value\n      (attribute_value) @injection.content\n      (#set! injection.language \"injection.content\")\n    )\n  )\n  (#eq? @_attr \"style\")\n  (#set! injection.language \"css\")\n)\n\n(\n  (attribute\n    (quoted_attribute_value\n      (attribute_value) @injection.content\n      (#set! injection.language \"injection.content\")\n    )\n  )\n  (#lua-match? @injection.content \"%${\")\n  (#offset! @injection.content 0 2 0 -1)\n  (#set! injection.language \"javascript\")\n)\n\n(\n  (attribute\n    (attribute_value) @injection.content\n    (#set! injection.language \"injection.content\")\n  )\n  (#lua-match? @injection.content \"%${\")\n  (#offset! @injection.content 0 2 0 -2)\n  (#set! injection.language \"javascript\")\n)\n\n(\n  (comment) @injection.content\n  (#set! injection.language \"injection.content\")\n  (#set! injection.language \"comment\")\n)\n\n(element\n  (_\n    (tag_name) @_tagname\n    (#eq? @_tagname \"input\")\n    (\n      (attribute\n        (attribute_name) @_attr\n        [\n          (quoted_attribute_value\n            (attribute_value) @injection.content\n            (#set! injection.language \"injection.content\")\n          )\n          (attribute_value) @injection.content\n          (#set! injection.language \"injection.content\")\n        ]\n        (#eq? @_attr \"pattern\")\n        (#set! injection.language \"regex\")\n      )\n    )\n  )\n)\n\n(attribute\n  (attribute_name) @_name\n  (#lua-match? @_name \"^on[a-z]+$\")\n  (quoted_attribute_value\n    (attribute_value) @injection.content\n    (#set! injection.language \"injection.content\")\n  )\n  (#set! injection.language \"javascript\")\n)\n\n(element\n  (start_tag\n    (tag_name) @_py_script\n  )\n  (text) @injection.content\n  (#set! injection.language \"injection.content\")\n  (#any-of? @_py_script \"py-script\"\"py-repl\")\n  (#set! injection.language \"python\")\n)\n\n(script_element\n  (start_tag\n    (attribute\n      (attribute_name) @_attr\n      (quoted_attribute_value\n        (attribute_value) @_type\n      )\n    )\n  )\n  (raw_text) @injection.content\n  (#set! injection.language \"injection.content\")\n  (#eq? @_attr \"type\")\n  (#any-of? @_type \"pyscript\"\"py-script\")\n  (#set! injection.language \"python\")\n)\n\n(element\n  (start_tag\n    (tag_name) @_py_config\n  )\n  (text) @injection.content\n  (#set! injection.language \"injection.content\")\n  (#eq? @_py_config \"py-config\")\n  (#set! injection.language \"toml\")\n)";

fn main() {
    std::fs::write("asd.scm", HTML_INJECTIONS).unwrap();

    let languages = ConfiguredLanguages::try_configure(
        &ParserProviderGit::all(),
        syntastica_themes::one::dark(),
    )
    .unwrap();
    let mut highlighter = Highlighter::new();

    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    for (file_ext, code) in &examples {
        println!("\n\x1b[1m{file_ext}:\x1b[0m\n{}", "-".repeat(50));
        example(&languages, &mut highlighter, code, file_ext);
        println!("{}", "-".repeat(50))
    }
}

fn example(
    languages: &ConfiguredLanguages,
    highlighter: &mut Highlighter,
    code: &str,
    file_extension: &str,
) {
    println!(
        "{}",
        syntastica::render(
            &syntastica::process(
                code.trim(),
                file_extension,
                languages,
                |lang_name| ParserProviderGit::all().for_injection(lang_name),
                highlighter
            )
            .unwrap(),
            &mut TerminalRenderer
        )
    );
}
