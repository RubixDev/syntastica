//! The 'omni' theme collection in this module was extracted from <https://github.com/yonlu/omni> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn omni() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(225, 225, 230), Some(Color::new(25, 22, 34)), false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, true)),
        ("comment".into(), Style::new(Color::new(90, 75, 129), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("constant".into(), Style::new(Color::new(161, 239, 228), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("field".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("function.builtin".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("macro".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("namespace".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(120, 209, 225), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(120, 209, 225), None, false, false, true, false)),
        ("storageclass".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(90, 75, 129), None, false, false, true, false)),
        ("text.reference".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(103, 228, 128), None, false, false, false, false)),
        ("text.todo".into(), Style::new(Color::new(152, 139, 199), None, false, false, false, true)),
        ("type".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(231, 222, 121), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(230, 31, 68), None, false, false, false, false)),
    ]))
}
