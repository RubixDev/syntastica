//! The 'abscs' theme collection in this module was extracted from <https://github.com/Abstract-IDE/Abstract-cs> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn abscs() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(255, 250, 222), Some(Color::new(6, 6, 6)), false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(1, 160, 245), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(80, 193, 110), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(157, 151, 151), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(92, 77, 77), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(80, 193, 110), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(237, 114, 46), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(237, 114, 46), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(255, 255, 255), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(237, 114, 46), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(249, 38, 114), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(227, 0, 34), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(136, 136, 136), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(214, 142, 178), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(255, 255, 255), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(255, 204, 102), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(249, 38, 114), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(249, 38, 114), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(204, 204, 204), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(227, 0, 34), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(249, 38, 114), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(207, 92, 54), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(157, 151, 151), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(157, 151, 151), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(23, 140, 148), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(92, 77, 77), None, false, false, true, false)),
        ("text.reference".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(255, 255, 153), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(255, 255, 255), None, false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(128, 160, 255), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(128, 160, 255), None, true, false, false, false)),
        ("type".into(), Style::new(Color::new(29, 145, 139), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(90, 209, 170), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(255, 250, 222), None, false, false, false, false)),
    ]))
}
