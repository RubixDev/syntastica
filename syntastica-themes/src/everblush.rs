//! The 'everblush' theme collection in this module was extracted from <https://github.com/Everblush/nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn everblush() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(218, 218, 218), Some(Color::new(20, 27, 30)), false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("attribute.builtin".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(218, 218, 218), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(64, 71, 73), None, false, false, true, false)),
        ("comment.error".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("comment.note".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("comment.todo".into(), Style::new(Color::new(229, 116, 116), Some(Color::new(20, 27, 30)), false, false, false, false)),
        ("comment.warning".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("conditional".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("diff.delta".into(), Style::new(Color::new(140, 248, 247), None, false, false, false, false)),
        ("diff.minus".into(), Style::new(Color::new(255, 192, 185), None, false, false, false, false)),
        ("diff.plus".into(), Style::new(Color::new(179, 246, 192), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(45, 52, 55), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(45, 52, 55), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(103, 203, 231), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(140, 207, 126), None, false, false, false, false)),
        ("ibl.indent.char.1".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("ibl.scope.char.1".into(), Style::new(Color::new(35, 42, 45), Some(Color::new(20, 27, 30)), false, false, false, false)),
        ("ibl.whitespace.char.1".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(239, 126, 126), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(196, 127, 213), None, false, false, false, false)),
        ("keyword.function".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(113, 186, 242), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("label".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("markup".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("markup.heading".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, true)),
        ("markup.link".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(113, 186, 242), None, false, false, false, false)),
        ("module".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("module.builtin".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(239, 126, 126), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("number.float".into(), Style::new(Color::new(196, 127, 213), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("parameter.reference".into(), Style::new(Color::new(239, 126, 126), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(218, 218, 218), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("repeat".into(), Style::new(Color::new(244, 214, 122), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(140, 207, 126), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(140, 207, 126), None, false, false, false, false)),
        ("string.regexp".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("string.special.url".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("symbol".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(229, 116, 116), None, false, false, false, false)),
        ("tag.builtin".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("text".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(45, 52, 55), None, false, false, false, false)),
        ("text.emphasis".into(), Style::new(Color::new(179, 185, 184), None, false, false, true, false)),
        ("text.environment.name".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("text.environtment".into(), Style::new(Color::new(196, 127, 213), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(140, 207, 126), None, false, false, true, false)),
        ("text.math".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(45, 52, 55), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
        ("text.strike".into(), Style::new(Color::new(179, 185, 184), None, false, true, false, false)),
        ("text.strong".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, true)),
        ("text.title".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, true)),
        ("text.underline".into(), Style::new(Color::new(196, 127, 213), None, true, false, false, false)),
        ("text.uri".into(), Style::new(Color::new(229, 199, 107), None, true, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(35, 42, 45), Some(Color::new(229, 116, 116)), false, false, false, false)),
        ("type".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(229, 199, 107), None, false, false, false, false)),
        ("variable".into(), Style::new(Color::new(179, 185, 184), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(103, 176, 232), None, false, false, false, false)),
        ("variable.parameter.builtin".into(), Style::new(Color::new(108, 191, 191), None, false, false, false, false)),
    ]))
}
