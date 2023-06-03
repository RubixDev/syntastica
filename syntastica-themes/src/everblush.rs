//! The everblush theme collection in this module was extracted from <https://github.com/Everblush/nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn everblush() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(20, 27, 30)),
        ("attribute".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(218, 218, 218), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(64, 71, 73), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(45, 52, 55), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(45, 52, 55), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(103, 203, 231), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(140, 207, 126), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(239, 126, 126), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(196, 127, 213), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(113, 186, 242), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(113, 186, 242), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(239, 126, 126), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("parameter.reference".to_owned(), Style::new(Color::new(239, 126, 126), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(218, 218, 218), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(244, 214, 122), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(140, 207, 126), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(140, 207, 126), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("text".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(45, 52, 55), false, false, false, false)),
        ("text.emphasis".to_owned(), Style::new(Color::new(179, 185, 184), false, false, true, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, false)),
        ("text.environtment".to_owned(), Style::new(Color::new(196, 127, 213), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(140, 207, 126), false, false, true, false)),
        ("text.math".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(45, 52, 55), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("text.strike".to_owned(), Style::new(Color::new(179, 185, 184), false, true, false, false)),
        ("text.strong".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, true)),
        ("text.title".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(229, 116, 116), false, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(196, 127, 213), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(229, 199, 107), true, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(35, 42, 45), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(229, 199, 107), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(108, 191, 191), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(179, 185, 184), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(103, 176, 232), false, false, false, false)),
    ]))
}
