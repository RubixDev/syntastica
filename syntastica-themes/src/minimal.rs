//! The 'minimal' theme collection in this module was extracted from <https://github.com/Yazeed1s/minimal.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn minimal() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(223, 224, 234), Some(Color::new(25, 27, 32)), false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(77, 82, 100), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(216, 149, 199), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(223, 224, 234), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(223, 224, 234), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(148, 221, 142), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(148, 221, 142), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(224, 130, 141), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(223, 224, 234), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(232, 90, 132), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(233, 210, 108), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(77, 82, 100), None, false, false, true, false)),
        ("text.reference".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(81, 86, 105), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(233, 210, 108), Some(Color::new(255, 255, 0)), false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(128, 160, 255), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(128, 160, 255), None, true, false, false, false)),
        ("type".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(126, 183, 230), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(207, 208, 215), None, false, false, false, false)),
    ]))
}
