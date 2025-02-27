//! The 'blue_moon' theme collection in this module was extracted from <https://github.com/kyazdani42/blue-moon> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn blue_moon() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(251, 251, 251), Some(Color::new(27, 30, 43)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(185, 163, 235), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(180, 196, 180), None, false, false, false, true)),
        ("comment".into(), Style::new(Color::new(103, 110, 150), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(208, 97, 120), Some(Color::new(45, 37, 51)), false, false, false, true)),
        ("comment.note".into(), Style::new(Color::new(207, 207, 191), Some(Color::new(45, 48, 58)), false, false, false, true)),
        ("comment.todo".into(), Style::new(Color::new(185, 163, 235), None, false, false, false, true)),
        ("comment.warning".into(), Style::new(Color::new(185, 163, 235), None, false, false, false, true)),
        ("constant".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(185, 163, 235), None, false, false, false, false)),
        ("function.call".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("function.method".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("function.method.call".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(103, 110, 150), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("keyword.conditional".into(), Style::new(Color::new(149, 157, 203), None, false, false, true, false)),
        ("keyword.debug".into(), Style::new(Color::new(137, 187, 221), None, false, false, true, false)),
        ("keyword.directive".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("keyword.directive.define".into(), Style::new(Color::new(185, 163, 235), None, false, false, false, false)),
        ("keyword.exception".into(), Style::new(Color::new(149, 157, 203), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("keyword.import".into(), Style::new(Color::new(137, 187, 221), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("keyword.repeat".into(), Style::new(Color::new(149, 157, 203), None, false, false, true, false)),
        ("keyword.return".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(137, 187, 221), None, false, false, true, false)),
        ("markup".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("markup.link".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("markup.link.label".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("markup.link.url".into(), Style::new(Color::new(180, 196, 180), None, true, false, false, false)),
        ("markup.list".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("module".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(137, 187, 221), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(255, 255, 255), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(180, 196, 180), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(137, 187, 221), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(137, 187, 221), None, false, false, false, false)),
        ("string.special.symbol".into(), Style::new(Color::new(194, 176, 176), None, false, false, false, false)),
        ("string.special.url".into(), Style::new(Color::new(180, 196, 180), None, true, false, false, false)),
        ("tag".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(207, 207, 191), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(180, 180, 180), None, false, false, false, false)),
        ("variable.member".into(), Style::new(Color::new(184, 188, 243), None, false, false, false, false)),
        ("variable.parameter".into(), Style::new(Color::new(255, 255, 255), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(149, 157, 203), None, false, false, false, false)),
    ]))
}
