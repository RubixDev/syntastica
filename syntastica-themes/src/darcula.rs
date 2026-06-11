//! The 'darcula' theme collection in this module was extracted from <https://github.com/doums/darcula> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn darcula() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(169, 183, 198), Some(Color::new(43, 43, 43)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(152, 118, 170), None, false, false, true, false)),
        ("character".into(), Style::new(Color::new(106, 135, 89), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(128, 128, 128), None, false, false, false, false)),
        ("comment.error".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(168, 192, 35), None, false, false, true, false)),
        ("comment.warning".into(), Style::new(Color::new(252, 224, 148), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(152, 118, 170), None, false, false, true, false)),
        ("constant.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(255, 198, 109), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(96, 96, 96), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(96, 99, 102), Some(Color::new(49, 51, 53)), false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(96, 96, 96), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("markup.link".into(), Style::new(Color::new(169, 183, 198), None, true, false, false, false)),
        ("module".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(104, 151, 187), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(104, 151, 187), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(106, 135, 89), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("string.special.url".into(), Style::new(Color::new(169, 183, 198), None, true, false, false, false)),
        ("tag".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
    ]))
}
