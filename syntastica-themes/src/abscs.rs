//! The abscs theme collection in this module was extracted from <https://github.com/Abstract-IDE/Abstract-cs> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn abscs() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_fg".to_owned(), Style::color_only(255, 250, 222)),
        ("_bg".to_owned(), Style::color_only(6, 6, 6)),
        ("boolean".to_owned(), Style::new(Color::new(1, 160, 245), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(80, 193, 110), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(157, 151, 151), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(92, 77, 77), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(80, 193, 110), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(237, 114, 46), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(237, 114, 46), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(255, 255, 255), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(237, 114, 46), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(249, 38, 114), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(227, 0, 34), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(136, 136, 136), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(214, 142, 178), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(255, 255, 255), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(255, 204, 102), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(249, 38, 114), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(249, 38, 114), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(204, 204, 204), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(227, 0, 34), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(249, 38, 114), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(207, 92, 54), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(157, 151, 151), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(157, 151, 151), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(23, 140, 148), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(92, 77, 77), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(255, 255, 153), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(255, 255, 255), false, false, false, true)),
        ("text.underline".to_owned(), Style::new(Color::new(128, 160, 255), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(128, 160, 255), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(29, 145, 139), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(90, 209, 170), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(255, 250, 222), false, false, false, false)),
    ]))
}
