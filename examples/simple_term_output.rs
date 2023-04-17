use std::collections::BTreeMap;

use syntastica::{config::ThemeValue, renderer::TerminalRenderer};

fn main() {
    let code = r###"
fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    Regex::new(r"[a-fA-F0-9_]\s(.*)$");
}
"###
    .trim();
    println!(
        "{}",
        syntastica::highlight(code, "rs", &mut TerminalRenderer, theme().into()).unwrap()
    );

    let code = r###"
def fib(n: int) -> int:
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)
"###
    .trim();
    println!(
        "{}",
        syntastica::highlight(code, "py", &mut TerminalRenderer, theme().into()).unwrap()
    );
}

pub fn theme() -> BTreeMap<String, ThemeValue> {
    BTreeMap::from([
        ("black".to_owned(), ThemeValue::Simple("#181a1f".to_owned())),
        ("bg0".to_owned(), ThemeValue::Simple("#282c34".to_owned())),
        ("bg1".to_owned(), ThemeValue::Simple("#31353f".to_owned())),
        ("bg2".to_owned(), ThemeValue::Simple("#393f4a".to_owned())),
        ("bg3".to_owned(), ThemeValue::Simple("#3b3f4c".to_owned())),
        ("bg_d".to_owned(), ThemeValue::Simple("#21252b".to_owned())),
        (
            "bg_blue".to_owned(),
            ThemeValue::Simple("#73b8f1".to_owned()),
        ),
        (
            "bg_yellow".to_owned(),
            ThemeValue::Simple("#ebd09c".to_owned()),
        ),
        ("fg".to_owned(), ThemeValue::Simple("#abb2bf".to_owned())),
        (
            "purple".to_owned(),
            ThemeValue::Simple("#c678dd".to_owned()),
        ),
        ("green".to_owned(), ThemeValue::Simple("#98c379".to_owned())),
        (
            "orange".to_owned(),
            ThemeValue::Simple("#d19a66".to_owned()),
        ),
        ("blue".to_owned(), ThemeValue::Simple("#61afef".to_owned())),
        (
            "yellow".to_owned(),
            ThemeValue::Simple("#e5c07b".to_owned()),
        ),
        ("cyan".to_owned(), ThemeValue::Simple("#56b6c2".to_owned())),
        ("red".to_owned(), ThemeValue::Simple("#e86671".to_owned())),
        ("grey".to_owned(), ThemeValue::Simple("#5c6370".to_owned())),
        (
            "light_grey".to_owned(),
            ThemeValue::Simple("#848b98".to_owned()),
        ),
        (
            "dark_cyan".to_owned(),
            ThemeValue::Simple("#2b6f77".to_owned()),
        ),
        (
            "dark_red".to_owned(),
            ThemeValue::Simple("#993939".to_owned()),
        ),
        (
            "dark_yellow".to_owned(),
            ThemeValue::Simple("#93691d".to_owned()),
        ),
        (
            "dark_purple".to_owned(),
            ThemeValue::Simple("#8a3fa0".to_owned()),
        ),
        (
            "diff_add".to_owned(),
            ThemeValue::Simple("#31392b".to_owned()),
        ),
        (
            "diff_delete".to_owned(),
            ThemeValue::Simple("#382b2c".to_owned()),
        ),
        (
            "diff_change".to_owned(),
            ThemeValue::Simple("#1c3448".to_owned()),
        ),
        (
            "diff_text".to_owned(),
            ThemeValue::Simple("#2c5372".to_owned()),
        ),
        (
            "annotation".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "attribute".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "boolean".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "character".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "comment".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("grey".to_owned()),
            },
        ),
        (
            "conditional".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "constant".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constant.builtin".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constant.macro".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "constructor".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("yellow".to_owned()),
            },
        ),
        ("error".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "exception".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("field".to_owned(), ThemeValue::Simple("$cyan".to_owned())),
        ("float".to_owned(), ThemeValue::Simple("$orange".to_owned())),
        (
            "function".to_owned(),
            ThemeValue::Simple("$blue".to_owned()),
        ),
        (
            "function.builtin".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "function.macro".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "include".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword.function".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "keyword.operator".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("label".to_owned(), ThemeValue::Simple("$red".to_owned())),
        ("method".to_owned(), ThemeValue::Simple("$blue".to_owned())),
        (
            "namespace".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        ("none".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "number".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        ("operator".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "parameter".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "parameter.reference".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "property".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "punctuation.delimiter".to_owned(),
            ThemeValue::Simple("$light_grey".to_owned()),
        ),
        (
            "punctuation.bracket".to_owned(),
            ThemeValue::Simple("$light_grey".to_owned()),
        ),
        (
            "punctuation.special".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        (
            "repeat".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("string".to_owned(), ThemeValue::Simple("$green".to_owned())),
        (
            "string.regex".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "string.escape".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        ("symbol".to_owned(), ThemeValue::Simple("$cyan".to_owned())),
        ("tag".to_owned(), ThemeValue::Simple("$purple".to_owned())),
        (
            "tag.delimiter".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("text".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "text.strong".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.emphasis".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.underline".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: true,
                strikethrough: false,
                italic: false,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.strike".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: true,
                italic: false,
                bold: false,
                link: Some("fg".to_owned()),
            },
        ),
        (
            "text.title".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: false,
                bold: true,
                link: Some("orange".to_owned()),
            },
        ),
        (
            "text.literal".to_owned(),
            ThemeValue::Simple("$green".to_owned()),
        ),
        (
            "text.uri".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: true,
                strikethrough: false,
                italic: false,
                bold: false,
                link: Some("cyan".to_owned()),
            },
        ),
        (
            "text.todo".to_owned(),
            ThemeValue::Extended {
                color: None,
                underline: false,
                strikethrough: false,
                italic: true,
                bold: false,
                link: Some("red".to_owned()),
            },
        ),
        ("text.math".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "text.reference".to_owned(),
            ThemeValue::Simple("$blue".to_owned()),
        ),
        (
            "text.enviroment".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.enviroment.name".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        ("note".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("warning".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("danger".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        ("type".to_owned(), ThemeValue::Simple("$yellow".to_owned())),
        (
            "type.builtin".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "type.qualifier".to_owned(),
            ThemeValue::Simple("$purple".to_owned()),
        ),
        ("variable".to_owned(), ThemeValue::Simple("$fg".to_owned())),
        (
            "variable.builtin".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "string.special.grammar".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "symbol.grammar.pascal".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        (
            "symbol.grammar.camel".to_owned(),
            ThemeValue::Simple("$cyan".to_owned()),
        ),
        (
            "symbol.grammar.upper".to_owned(),
            ThemeValue::Simple("$orange".to_owned()),
        ),
        (
            "symbol.grammar.lower".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "storageclass.lifetime".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
        (
            "tag.attribute".to_owned(),
            ThemeValue::Simple("$yellow".to_owned()),
        ),
        (
            "text.environment".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.environment.name".to_owned(),
            ThemeValue::Simple("$fg".to_owned()),
        ),
        (
            "text.diff.add".to_owned(),
            ThemeValue::Simple("$green".to_owned()),
        ),
        (
            "text.diff.delete".to_owned(),
            ThemeValue::Simple("$red".to_owned()),
        ),
    ])
}
