//! The moonlight themes collection in this module was extracted from <https://github.com/shaunsingh/moonlight.nvim> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn moonlight() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(33, 35, 55)),
        ("variable".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(246, 127, 129), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(116, 134, 214), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(45, 244, 192), false, false, false, true)),
        ("text.uri".to_owned(), Style::new(Color::new(128, 203, 196), true, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(128, 203, 196), true, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(255, 199, 119), false, false, true, true)),
        ("comment".to_owned(), Style::new(Color::new(116, 134, 214), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(255, 199, 119), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(236, 178, 240), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(236, 178, 240), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(45, 244, 192), false, false, true, false)),
        ("string.escape".to_owned(), Style::new(Color::new(236, 178, 240), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(236, 178, 240), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(246, 127, 129), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(236, 178, 240), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(246, 127, 129), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(246, 127, 129), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(4, 209, 249), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(4, 209, 249), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(185, 148, 241), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(64, 255, 255), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(4, 209, 249), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(180, 164, 244), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
    ]))
}
