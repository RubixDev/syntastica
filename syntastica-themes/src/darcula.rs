//! The darcula theme collection in this module was extracted from <https://github.com/doums/darcula> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn darcula() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(43, 43, 43)),
        ("boolean".to_owned(), Style::new(Color::new(152, 118, 170), false, false, true, false)),
        ("character".to_owned(), Style::new(Color::new(106, 135, 89), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(128, 128, 128), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(152, 118, 170), false, false, true, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(102, 109, 117), false, false, true, false)),
        ("define".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(104, 151, 187), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(255, 198, 109), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(255, 198, 109), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(104, 151, 187), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(106, 135, 89), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(128, 128, 128), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(187, 181, 41), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(168, 192, 35), false, false, true, false)),
        ("text.underline".to_owned(), Style::new(Color::new(169, 183, 198), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(169, 183, 198), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(204, 120, 50), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(185, 188, 209), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(169, 183, 198), false, false, false, false)),
    ]))
}
