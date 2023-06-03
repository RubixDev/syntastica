//! The zephyr themes collection in this module was extracted from <https://github.com/glepnir/zephyr-nvim> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn zephyr() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(38, 42, 51)),
        ("text.literal".to_owned(), Style::new(Color::new(115, 121, 126), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(203, 166, 247), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(115, 121, 126), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(187, 194, 207), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(54, 208, 224), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(203, 166, 247), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(203, 166, 247), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(250, 183, 149), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(199, 134, 101), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(54, 208, 224), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(198, 120, 221), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(198, 120, 221), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(209, 109, 158), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(175, 215, 0), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(233, 86, 120), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(233, 86, 120), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(203, 166, 247), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(203, 166, 247), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(255, 135, 0), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(26, 188, 156), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(199, 134, 101), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(97, 175, 239), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(54, 208, 224), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(175, 215, 0), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(247, 187, 59), false, false, false, false)),
    ]))
}
