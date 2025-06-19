//! The 'minimal' theme collection in this module was extracted from <https://github.com/Yazeed1s/minimal.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn minimal() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(223, 224, 234), Some(Color::new(25, 27, 32)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(77, 82, 100), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(217, 85, 85), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, true)),
        ("comment.warning".into(), Style::new(Color::new(227, 154, 101), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(216, 149, 199), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(148, 221, 142), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(96, 105, 120), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(85, 91, 108), Some(Color::new(30, 32, 38)), false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(96, 105, 120), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, true)),
        ("module".into(), Style::new(Color::new(148, 221, 142), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(223, 224, 234), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
    ]))
}
