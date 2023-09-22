//! The boo theme collection in this module was extracted from <https://github.com/rockerBOO/boo-colorscheme-nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn boo() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_fg".to_owned(), Style::color_only(225, 217, 234)),
        ("_bg".to_owned(), Style::color_only(15, 15, 16)),
        ("attribute".to_owned(), Style::new(Color::new(114, 84, 120), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(185, 158, 231), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(132, 157, 162), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(63, 52, 66), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(102, 145, 154), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(83, 117, 126), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(125, 108, 130), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(66, 145, 173), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(125, 108, 130), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(101, 74, 150), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(205, 116, 156), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(132, 157, 162), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(101, 74, 150), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(125, 162, 169), false, false, true, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(58, 114, 114), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(77, 162, 179), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(125, 162, 169), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(101, 74, 150), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(104, 76, 154), false, false, true, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(104, 76, 154), false, false, true, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(104, 76, 154), false, false, true, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(104, 76, 154), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(99, 176, 176), false, false, true, false)),
        ("macro".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(77, 162, 179), false, false, true, false)),
        ("method.call".to_owned(), Style::new(Color::new(77, 162, 179), false, false, true, false)),
        ("namespace".to_owned(), Style::new(Color::new(161, 180, 184), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(217, 214, 207), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(101, 101, 104), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(169, 209, 223), false, false, false, false)),
        ("parameter.reference".to_owned(), Style::new(Color::new(169, 209, 223), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(132, 157, 162), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(143, 143, 183), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(101, 101, 104), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(97, 97, 108), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(115, 127, 141), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(83, 117, 126), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(218, 243, 243), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(218, 243, 243), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(218, 243, 243), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(218, 243, 243), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(133, 193, 193), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(115, 141, 141), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(59, 70, 73), false, false, false, false)),
        ("text".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.emphassis".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.environment".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.math".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.strike".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.string".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(63, 52, 66), false, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(228, 220, 236), false, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(72, 133, 133), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(116, 138, 144), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(99, 176, 176), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(148, 194, 210), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(59, 99, 146), false, false, false, false)),
    ]))
}
