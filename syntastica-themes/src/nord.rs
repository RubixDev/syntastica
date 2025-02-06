//! The 'nord' theme collection in this module was extracted from <https://github.com/shaunsingh/nord.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn nord() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(216, 222, 233), Some(Color::new(46, 52, 64)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, true)),
        ("character".into(), Style::new(Color::new(163, 190, 140), None, false, false, true, false)),
        ("character.special".into(), Style::new(Color::new(235, 203, 139), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(97, 110, 136), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(191, 97, 106), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(94, 129, 172), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(235, 203, 139), None, false, false, true, true)),
        ("comment.warning".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("conditional".into(), Style::new(Color::new(129, 161, 193), None, false, false, true, false)),
        ("constant".into(), Style::new(Color::new(235, 203, 139), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(143, 188, 187), None, false, false, false, true)),
        ("constant.macro".into(), Style::new(Color::new(143, 188, 187), None, false, false, false, true)),
        ("constructor".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("error".into(), Style::new(Color::new(191, 97, 106), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(216, 222, 233), None, false, false, true, false)),
        ("float".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("function.builtin".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("funtion.macro".into(), Style::new(Color::new(143, 188, 187), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(59, 66, 82), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(97, 110, 136), None, false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(59, 66, 82), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(129, 161, 193), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("keyword.operator".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("keyword.return".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("label".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(163, 190, 140), None, false, false, false, true)),
        ("markup.link".into(), Style::new(Color::new(163, 190, 140), None, true, false, false, false)),
        ("method".into(), Style::new(Color::new(136, 192, 208), None, false, false, true, false)),
        ("module".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(216, 222, 233), None, false, false, true, false)),
        ("number".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(94, 129, 172), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(94, 129, 172), None, false, false, true, false)),
        ("punctuation".into(), Style::new(Color::new(236, 239, 244), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(136, 192, 208), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(136, 192, 208), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(136, 192, 208), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(129, 161, 193), None, false, false, true, false)),
        ("string".into(), Style::new(Color::new(163, 190, 140), None, false, false, true, false)),
        ("string.escape".into(), Style::new(Color::new(180, 142, 173), None, false, false, true, false)),
        ("string.regex".into(), Style::new(Color::new(143, 188, 187), None, false, false, true, false)),
        ("string.regexp".into(), Style::new(Color::new(235, 203, 139), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(235, 203, 139), None, false, false, false, false)),
        ("string.special.url".into(), Style::new(Color::new(163, 190, 140), None, true, false, false, false)),
        ("symbol".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, false)),
        ("text".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("text.emphasis".into(), Style::new(Color::new(94, 129, 172), None, false, false, true, false)),
        ("text.literal".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
        ("text.math".into(), Style::new(Color::new(143, 188, 187), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(180, 142, 173), None, false, false, false, true)),
        ("text.strike".into(), Style::new(Color::new(216, 222, 233), None, false, true, false, false)),
        ("text.strong".into(), Style::new(Color::new(94, 129, 172), None, false, false, false, true)),
        ("text.title".into(), Style::new(Color::new(94, 129, 172), None, false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(216, 222, 233), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(163, 190, 140), None, true, false, false, false)),
        ("type".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(129, 161, 193), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, true)),
        ("variable.builtin".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, true)),
        ("variable.global".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, true)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(216, 222, 233), None, false, false, false, false)),
    ]))
}
