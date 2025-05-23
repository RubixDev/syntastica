//! The 'vscode' theme collection in this module was extracted from <https://github.com/Mofiqul/vscode.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn dark() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(212, 212, 212), Some(Color::new(31, 31, 31)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(106, 153, 85), None, false, false, false, false)),
        ("comment.error".into(), Style::new(Color::new(244, 71, 71), None, false, false, false, true)),
        ("comment.note".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, true)),
        ("comment.todo".into(), Style::new(Color::new(215, 186, 125), Some(Color::new(31, 31, 31)), false, false, false, true)),
        ("comment.warning".into(), Style::new(Color::new(215, 186, 125), None, false, false, false, true)),
        ("conditional".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(79, 193, 255), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("constructor.python".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("decorator".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("error".into(), Style::new(Color::new(244, 71, 71), None, false, false, false, false)),
        ("event".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(181, 206, 168), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("function.method".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("interface".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("keyword.conditional".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("keyword.exception".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("keyword.import".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("keyword.repeat".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(215, 186, 125), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, true)),
        ("markup.italic".into(), Style::new(Color::new(212, 212, 212), None, false, false, true, false)),
        ("markup.link.label".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("markup.link.url".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("markup.list.checked".into(), Style::new(Color::new(215, 186, 125), Some(Color::new(31, 31, 31)), false, false, false, true)),
        ("markup.list.unchecked".into(), Style::new(Color::new(215, 186, 125), Some(Color::new(31, 31, 31)), false, false, false, true)),
        ("markup.raw".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("markup.raw.markdown".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("markup.raw.markdown_inline".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("markup.strikethrough".into(), Style::new(Color::new(212, 212, 212), None, false, true, false, false)),
        ("markup.strong".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, true)),
        ("markup.underline".into(), Style::new(Color::new(215, 186, 125), None, true, false, false, false)),
        ("method".into(), Style::new(Color::new(220, 220, 170), None, false, false, false, false)),
        ("modifier".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("module".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(215, 186, 125), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(181, 206, 168), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(181, 206, 168), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("regexp".into(), Style::new(Color::new(244, 71, 71), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(197, 134, 192), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("stringEscape".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, true)),
        ("structure".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(128, 128, 128), None, false, false, false, false)),
        ("text".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(244, 71, 71), None, false, false, false, true)),
        ("text.emphasis".into(), Style::new(Color::new(212, 212, 212), None, false, false, true, false)),
        ("text.literal".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, true)),
        ("text.strike".into(), Style::new(Color::new(212, 212, 212), None, false, true, false, false)),
        ("text.strong".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, true)),
        ("text.title".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(215, 186, 125), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(215, 186, 125), None, false, false, false, true)),
        ("textReference".into(), Style::new(Color::new(206, 145, 120), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("type.builtin.tsx".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("type.builtin.typescript".into(), Style::new(Color::new(78, 201, 176), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(86, 156, 214), None, false, false, false, false)),
        ("variable.member".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("variable.parameter".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(215, 186, 125), None, false, false, false, false)),
        ("variable.parameter.reference".into(), Style::new(Color::new(156, 220, 254), None, false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn light() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(52, 52, 52), Some(Color::new(255, 255, 255)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(0, 128, 0), None, false, false, false, false)),
        ("comment.error".into(), Style::new(Color::new(255, 0, 0), None, false, false, false, true)),
        ("comment.note".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, true)),
        ("comment.todo".into(), Style::new(Color::new(128, 0, 0), Some(Color::new(255, 255, 255)), false, false, false, true)),
        ("comment.warning".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, true)),
        ("conditional".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("constructor.python".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("decorator".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("error".into(), Style::new(Color::new(255, 0, 0), None, false, false, false, false)),
        ("event".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(9, 134, 88), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("function.method".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(206, 206, 206), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(9, 134, 88), Some(Color::new(255, 255, 255)), false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(206, 206, 206), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("interface".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("keyword.conditional".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("keyword.exception".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("keyword.import".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("keyword.repeat".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, true)),
        ("markup.italic".into(), Style::new(Color::new(52, 52, 52), None, false, false, true, false)),
        ("markup.link.label".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("markup.link.url".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("markup.list.checked".into(), Style::new(Color::new(128, 0, 0), Some(Color::new(255, 255, 255)), false, false, false, true)),
        ("markup.list.unchecked".into(), Style::new(Color::new(128, 0, 0), Some(Color::new(255, 255, 255)), false, false, false, true)),
        ("markup.raw".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("markup.raw.markdown".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("markup.raw.markdown_inline".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("markup.strikethrough".into(), Style::new(Color::new(52, 52, 52), None, false, true, false, false)),
        ("markup.strong".into(), Style::new(Color::new(0, 0, 128), None, false, false, false, true)),
        ("markup.underline".into(), Style::new(Color::new(128, 0, 0), None, true, false, false, false)),
        ("method".into(), Style::new(Color::new(121, 94, 38), None, false, false, false, false)),
        ("modifier".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("module".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(9, 134, 88), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(9, 134, 88), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("regexp".into(), Style::new(Color::new(255, 0, 0), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(175, 0, 219), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(199, 46, 15), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("stringEscape".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, true)),
        ("structure".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(0, 0, 0), None, false, false, false, false)),
        ("text".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(255, 0, 0), None, false, false, false, true)),
        ("text.emphasis".into(), Style::new(Color::new(52, 52, 52), None, false, false, true, false)),
        ("text.literal".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, true)),
        ("text.strike".into(), Style::new(Color::new(52, 52, 52), None, false, true, false, false)),
        ("text.strong".into(), Style::new(Color::new(0, 0, 128), None, false, false, false, true)),
        ("text.title".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(128, 0, 0), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(52, 52, 52), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, true)),
        ("textReference".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("type.builtin.tsx".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("type.builtin.typescript".into(), Style::new(Color::new(22, 130, 93), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(0, 0, 255), None, false, false, false, false)),
        ("variable.member".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("variable.parameter".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(128, 0, 0), None, false, false, false, false)),
        ("variable.parameter.reference".into(), Style::new(Color::new(4, 81, 165), None, false, false, false, false)),
    ]))
}
