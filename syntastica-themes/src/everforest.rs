//! The everforest themes collection in this module was extracted from <https://github.com/sainnhe/everforest> using `auto_extract.py`

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn dark() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(41, 37, 34)),
        ("text.uri".to_owned(), Style::new(Color::new(163, 169, 206), true, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(193, 167, 142), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(189, 129, 131), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(236, 225, 215), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(137, 179, 182), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(179, 128, 176), false, false, true, false)),
        ("string.escape".to_owned(), Style::new(Color::new(127, 145, 178), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(179, 128, 176), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(236, 225, 215), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("storageclass.lifetime".to_owned(), Style::new(Color::new(137, 179, 182), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(139, 116, 73), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(236, 225, 215), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(207, 155, 194), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(236, 225, 215), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(236, 225, 215), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(137, 179, 182), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(179, 128, 176), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(207, 155, 194), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(193, 167, 142), false, false, false, true)),
        ("comment".to_owned(), Style::new(Color::new(193, 167, 142), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(139, 116, 73), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(179, 128, 176), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(163, 169, 206), false, false, true, false)),
        ("string.special".to_owned(), Style::new(Color::new(137, 179, 182), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(163, 169, 206), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(137, 179, 182), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(207, 155, 194), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(236, 225, 215), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(212, 119, 102), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(228, 155, 93), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(123, 150, 149), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(123, 150, 149), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(236, 225, 215), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(133, 182, 149), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(235, 192, 109), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn light() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(241, 241, 241)),
        ("text.title".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("text.todo".to_owned(), Style::new(Color::new(125, 102, 88), false, false, false, true)),
        ("comment".to_owned(), Style::new(Color::new(125, 102, 88), false, false, true, false)),
        ("punctuation".to_owned(), Style::new(Color::new(204, 164, 120), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(190, 121, 187), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(70, 90, 164), false, false, true, false)),
        ("string.special".to_owned(), Style::new(Color::new(61, 101, 104), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(70, 90, 164), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(61, 101, 104), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(144, 65, 128), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(144, 65, 128), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(84, 67, 58), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(191, 0, 33), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(115, 151, 151), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(115, 151, 151), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(84, 67, 58), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(190, 121, 187), false, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(70, 90, 164), true, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(125, 102, 88), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(199, 123, 139), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(84, 67, 58), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(61, 101, 104), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(190, 121, 187), false, false, true, false)),
        ("string.escape".to_owned(), Style::new(Color::new(120, 146, 189), false, false, false, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(190, 121, 187), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(84, 67, 58), false, false, true, false)),
        ("function.macro".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(188, 92, 0), false, false, false, false)),
        ("storageclass.lifetime".to_owned(), Style::new(Color::new(61, 101, 104), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(61, 101, 104), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(204, 164, 120), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(58, 104, 74), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(84, 67, 58), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(84, 67, 58), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(144, 65, 128), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(160, 109, 0), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(84, 67, 58), false, false, false, false)),
    ]))
}
