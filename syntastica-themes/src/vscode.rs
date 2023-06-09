//! The vscode theme collection in this module was extracted from <https://github.com/Mofiqul/vscode.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn dark() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(30, 30, 30)),
        ("annotation".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("attribute.builtin".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(106, 153, 85), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(79, 193, 254), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("decorator".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("error".to_owned(), Style::new(Color::new(244, 71, 71), false, false, false, false)),
        ("event".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(181, 206, 168), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("interface".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(220, 220, 170), false, false, false, false)),
        ("modifier".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(181, 206, 168), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("parameter.reference".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("regexp".to_owned(), Style::new(Color::new(244, 71, 71), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(197, 134, 192), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("stringEscape".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, true)),
        ("structure".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(128, 128, 128), false, false, false, false)),
        ("text".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(244, 71, 71), false, false, false, true)),
        ("text.emphasis".to_owned(), Style::new(Color::new(212, 212, 212), false, false, true, false)),
        ("text.literal".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("text.literal.markdown".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, true)),
        ("text.reference".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("text.strong".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, true)),
        ("text.title".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(215, 186, 125), false, false, false, true)),
        ("text.underline".to_owned(), Style::new(Color::new(215, 186, 125), false, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(215, 186, 125), false, false, false, true)),
        ("textReference".to_owned(), Style::new(Color::new(206, 145, 120), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(78, 201, 176), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(86, 156, 214), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(156, 220, 254), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn light() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(255, 255, 255)),
        ("annotation".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("attribute.builtin".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(199, 46, 15), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(0, 128, 0), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("decorator".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("error".to_owned(), Style::new(Color::new(255, 0, 0), false, false, false, false)),
        ("event".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(9, 134, 88), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("interface".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(121, 94, 38), false, false, false, false)),
        ("modifier".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(9, 134, 88), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("parameter.reference".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("regexp".to_owned(), Style::new(Color::new(255, 0, 0), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(175, 0, 219), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(199, 46, 15), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(199, 46, 15), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("stringEscape".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, true)),
        ("structure".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(0, 0, 0), false, false, false, false)),
        ("text".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(255, 0, 0), false, false, false, true)),
        ("text.emphasis".to_owned(), Style::new(Color::new(52, 52, 52), false, false, true, false)),
        ("text.literal".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("text.literal.markdown".to_owned(), Style::new(Color::new(199, 46, 15), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(199, 46, 15), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, true)),
        ("text.reference".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("text.strong".to_owned(), Style::new(Color::new(0, 0, 128), false, false, false, true)),
        ("text.title".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, true)),
        ("text.underline".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(52, 52, 52), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, true)),
        ("textReference".to_owned(), Style::new(Color::new(128, 0, 0), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(22, 130, 93), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(0, 0, 255), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(4, 81, 165), false, false, false, false)),
    ]))
}
