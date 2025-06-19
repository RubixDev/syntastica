//! The 'monochrome' theme collection in this module was extracted from <https://github.com/kdheepak/monochrome.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn monochrome() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(238, 238, 238), Some(Color::new(14, 14, 14)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("character".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(73, 73, 73), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("comment.warning".into(), Style::new(Color::new(252, 224, 148), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(53, 53, 53), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(73, 73, 73), None, false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(53, 53, 53), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("number.float".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("operator".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(224, 226, 234), None, false, false, false, false)),
    ]))
}
