//! The 'zephyr' theme collection in this module was extracted from <https://github.com/glepnir/zephyr-nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn zephyr() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(187, 194, 207), Some(Color::new(38, 42, 51)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(54, 208, 224), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(175, 215, 0), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(115, 121, 126), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("comment.warning".into(), Style::new(Color::new(252, 224, 148), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(54, 208, 224), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(63, 68, 74), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(63, 68, 74), None, false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(63, 68, 74), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(175, 215, 0), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, true)),
        ("module".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(199, 134, 101), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(199, 134, 101), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(209, 109, 158), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(187, 194, 207), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(250, 183, 149), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(26, 188, 156), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
    ]))
}
