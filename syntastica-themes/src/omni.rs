//! The omni theme collection in this module was extracted from <https://github.com/yonlu/omni> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn omni() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(25, 22, 34)),
        ("boolean".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(103, 228, 128), false, false, false, true)),
        ("comment".to_owned(), Style::new(Color::new(90, 75, 129), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("constant".to_owned(), Style::new(Color::new(161, 239, 228), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(120, 209, 225), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(152, 139, 199), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(120, 209, 225), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(152, 139, 199), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("field".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(120, 209, 225), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(152, 139, 199), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(120, 209, 225), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("macro".to_owned(), Style::new(Color::new(152, 139, 199), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("namespace".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(120, 209, 225), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(120, 209, 225), false, false, true, false)),
        ("storageclass".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(103, 228, 128), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(90, 75, 129), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(103, 228, 128), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(152, 139, 199), false, false, false, true)),
        ("type".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(231, 222, 121), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(230, 31, 68), false, false, false, false)),
    ]))
}
