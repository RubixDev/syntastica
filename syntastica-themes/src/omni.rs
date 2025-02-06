//! The 'omni' theme collection in this module was extracted from <https://github.com/yonlu/omni> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn omni() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(225, 225, 230), Some(Color::new(25, 22, 34)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, true)),
        ("comment".into(), Style::new(Color::new(90, 75, 129), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, true)),
        ("comment.warning".into(), Style::new(Color::new(252, 224, 148), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(161, 239, 228), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("function.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(90, 75, 129), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("markup".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, false)),
        ("module".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
    ]))
}
