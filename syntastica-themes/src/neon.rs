//! The neon theme collection in this module was extracted from <https://github.com/rafamadriz/neon> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn default() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(43, 45, 55)),
        ("character".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(169, 161, 225), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("debug".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(236, 190, 123), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(160, 201, 128), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(77, 181, 189), false, false, true, true)),
        ("text.underline".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn doom() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(40, 44, 52)),
        ("character".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(169, 161, 225), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("debug".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(236, 190, 123), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(160, 201, 128), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(77, 181, 189), false, false, true, true)),
        ("text.underline".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn dark() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(13, 17, 23)),
        ("character".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(169, 161, 225), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("debug".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(108, 182, 235), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(236, 190, 123), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(160, 201, 128), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(236, 114, 121), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(77, 181, 189), false, false, true, true)),
        ("text.underline".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(77, 181, 189), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(77, 181, 189), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(108, 182, 235), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn light() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(211, 211, 211)),
        ("character".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(147, 112, 219), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(34, 87, 160), false, false, true, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(34, 87, 160), false, false, true, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 102, 85), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(255, 102, 85), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(34, 87, 160), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(255, 102, 85), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(193, 132, 1), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(80, 161, 79), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(218, 133, 72), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(255, 102, 85), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(126, 130, 148), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(9, 151, 179), false, false, true, true)),
        ("text.underline".to_owned(), Style::new(Color::new(9, 151, 179), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(9, 151, 179), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(9, 151, 179), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(34, 87, 160), false, false, false, false)),
    ]))
}
