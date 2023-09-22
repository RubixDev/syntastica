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
        ("boolean".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("character".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(73, 73, 73), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(73, 73, 73), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(73, 73, 73), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("function".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(163, 163, 163), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(139, 139, 139), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, true)),
        ("operator".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(212, 212, 212), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(238, 238, 238), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(73, 73, 73), None, false, false, true, false)),
        ("text.reference".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(94, 94, 94), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(163, 163, 163), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(187, 187, 187), None, false, false, false, false)),
    ]))
}
