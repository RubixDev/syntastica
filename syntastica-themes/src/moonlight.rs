//! The 'moonlight' theme collection in this module was extracted from <https://github.com/shaunsingh/moonlight.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn moonlight() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(228, 243, 250), Some(Color::new(33, 35, 55)), false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(116, 134, 214), None, false, false, false, false)),
        ("conditional".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 199, 119), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(4, 209, 249), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(4, 209, 249), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(4, 209, 249), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(246, 127, 129), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(185, 148, 241), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(45, 244, 192), None, false, false, true, false)),
        ("string.escape".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(236, 178, 240), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(116, 134, 214), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(45, 244, 192), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(255, 199, 119), None, false, false, true, true)),
        ("text.underline".into(), Style::new(Color::new(128, 203, 196), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(128, 203, 196), None, true, false, false, false)),
        ("type".into(), Style::new(Color::new(180, 164, 244), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(64, 255, 255), None, false, false, false, false)),
    ]))
}
