//! The falcon themes collection in this module was extracted from <https://github.com/fenetikm/falcon> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn falcon() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("parameter.reference".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("note".to_owned(), Style::new(Color::new(2, 2, 33), false, false, true, false)),
        ("literal".to_owned(), Style::new(Color::new(200, 208, 227), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("error".to_owned(), Style::new(Color::new(255, 197, 82), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, true)),
        ("field".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("annotation".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(120, 120, 130), false, false, true, false)),
        ("text.reference".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(153, 164, 188), true, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(153, 164, 188), true, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(2, 2, 33), false, false, true, false)),
        ("comment".to_owned(), Style::new(Color::new(120, 120, 130), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, true)),
        ("constant.builtin".to_owned(), Style::new(Color::new(180, 180, 185), false, false, true, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(200, 208, 227), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(200, 208, 227), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(200, 208, 227), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(180, 180, 185), false, false, true, false)),
        ("float".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(255, 197, 82), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(255, 197, 82), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(255, 197, 82), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(255, 197, 82), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(200, 208, 227), false, false, false, false)),
        ("structure".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(153, 164, 188), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(180, 180, 185), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("title".to_owned(), Style::new(Color::new(207, 193, 178), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(223, 223, 229), false, false, false, false)),
        ("uri".to_owned(), Style::new(Color::new(153, 164, 188), true, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
        ("warning".to_owned(), Style::new(Color::new(188, 143, 63), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(255, 118, 26), false, false, false, false)),
    ]))
}
