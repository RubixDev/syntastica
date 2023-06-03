//! The minimal themes collection in this module was extracted from <https://github.com/Yazeed1s/minimal.nvim> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn minimal() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(25, 27, 32)),
        ("text.literal".to_owned(), Style::new(Color::new(77, 82, 100), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, true)),
        ("text.uri".to_owned(), Style::new(Color::new(128, 160, 255), true, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(128, 160, 255), true, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, true)),
        ("comment".to_owned(), Style::new(Color::new(77, 82, 100), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(216, 149, 199), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(233, 210, 108), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(224, 130, 141), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(224, 130, 141), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(148, 221, 142), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(148, 221, 142), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(223, 224, 234), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(223, 224, 234), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(232, 90, 132), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(126, 183, 230), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(223, 224, 234), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(224, 130, 141), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(81, 86, 105), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(207, 208, 215), false, false, false, false)),
    ]))
}
