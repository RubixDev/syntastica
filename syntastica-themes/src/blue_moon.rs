//! The blue_moon theme collection in this module was extracted from <https://github.com/kyazdani42/blue-moon> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn blue_moon() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(27, 30, 43)),
        ("boolean".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(180, 196, 180), false, false, false, true)),
        ("comment".to_owned(), Style::new(Color::new(103, 110, 150), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(149, 157, 203), false, false, true, false)),
        ("constant".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(137, 187, 221), false, false, true, false)),
        ("define".to_owned(), Style::new(Color::new(185, 163, 235), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(149, 157, 203), false, false, true, false)),
        ("field".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(185, 163, 235), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(137, 187, 221), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(149, 157, 203), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(149, 157, 203), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(137, 187, 221), false, false, true, false)),
        ("macro".to_owned(), Style::new(Color::new(185, 163, 235), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("method.call".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(255, 255, 255), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(137, 187, 221), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(255, 255, 255), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(149, 157, 203), false, false, true, false)),
        ("storageclass".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(180, 196, 180), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(137, 187, 221), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(137, 187, 221), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(194, 176, 176), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(184, 188, 243), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(103, 110, 150), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(185, 163, 235), false, false, false, true)),
        ("text.uri".to_owned(), Style::new(Color::new(180, 196, 180), true, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(185, 163, 235), false, false, false, true)),
        ("type".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(149, 157, 203), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(207, 207, 191), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(180, 180, 180), false, false, false, false)),
    ]))
}
