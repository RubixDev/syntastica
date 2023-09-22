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
        ("boolean".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(175, 215, 0), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(115, 121, 126), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(198, 120, 221), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(54, 208, 224), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(233, 86, 120), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(199, 134, 101), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(54, 208, 224), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(175, 215, 0), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(54, 208, 224), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(199, 134, 101), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(209, 109, 158), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(187, 194, 207), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(198, 120, 221), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(250, 183, 149), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(247, 187, 59), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(115, 121, 126), None, false, false, true, false)),
        ("text.reference".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(255, 135, 0), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(203, 166, 247), None, false, false, false, false)),
        ("type".into(), Style::new(Color::new(26, 188, 156), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(233, 86, 120), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(97, 175, 239), None, false, false, false, false)),
    ]))
}
