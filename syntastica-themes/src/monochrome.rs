//! The monochrome themes collection in this module was extracted from <https://github.com/kdheepak/monochrome.nvim> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn monochrome() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(14, 14, 14)),
        ("text.literal".to_owned(), Style::new(Color::new(73, 73, 73), false, false, true, false)),
        ("comment".to_owned(), Style::new(Color::new(73, 73, 73), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(73, 73, 73), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(94, 94, 94), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(94, 94, 94), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(94, 94, 94), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(163, 163, 163), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, true)),
        ("field".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(73, 73, 73), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(139, 139, 139), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(212, 212, 212), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, true)),
        ("float".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, true)),
        ("function".to_owned(), Style::new(Color::new(139, 139, 139), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(139, 139, 139), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(187, 187, 187), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(139, 139, 139), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(94, 94, 94), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(238, 238, 238), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(94, 94, 94), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(163, 163, 163), false, false, false, false)),
    ]))
}
