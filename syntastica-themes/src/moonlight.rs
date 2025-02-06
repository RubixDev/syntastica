//! The 'moonlight' theme collection in this module was extracted from <https://github.com/shaunsingh/moonlight.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn moonlight() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(228, 243, 250), Some(Color::new(33, 35, 55)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(116, 134, 214), None, false, false, false, false)),
        ("comment.error".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(255, 199, 119), None, false, false, true, true)),
        ("comment.warning".into(), Style::new(Color::new(252, 224, 148), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 199, 119), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(4, 209, 249), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(81, 87, 114), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(89, 99, 153), None, false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(81, 87, 114), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(45, 244, 192), None, false, false, false, true)),
        ("markup.link".into(), Style::new(Color::new(128, 203, 196), None, true, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(166, 219, 255), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(45, 244, 192), None, false, false, true, false)),
        ("string.escape".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("string.special.url".into(), Style::new(Color::new(128, 203, 196), None, true, false, false, false)),
        ("tag".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
    ]))
}
