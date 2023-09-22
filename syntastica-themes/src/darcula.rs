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
        ("boolean".into(), Style::new(Color::new(152, 118, 170), None, false, false, true, false)),
        ("character".into(), Style::new(Color::new(106, 135, 89), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(128, 128, 128), None, false, false, false, false)),
        ("conditional".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(152, 118, 170), None, false, false, true, false)),
        ("constant.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(102, 109, 117), None, false, false, true, false)),
        ("define".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(104, 151, 187), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(255, 198, 109), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(255, 198, 109), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(104, 151, 187), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(106, 135, 89), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(128, 128, 128), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(187, 181, 41), None, false, false, false, false)),
        ("text.todo".into(), Style::new(Color::new(168, 192, 35), None, false, false, true, false)),
        ("text.underline".into(), Style::new(Color::new(169, 183, 198), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(169, 183, 198), None, true, false, false, false)),
        ("type".into(), Style::new(Color::new(204, 120, 50), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(185, 188, 209), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(169, 183, 198), None, false, false, false, false)),
    ]))
}
