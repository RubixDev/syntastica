//! The 'tokyo' theme collection in this module was extracted from <https://github.com/folke/tokyonight.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn storm() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(192, 202, 245), Some(Color::new(36, 40, 59)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(86, 95, 137), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("constructor.tsx".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("function.call".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("keyword.coroutine".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("label".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("method.call".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(224, 175, 104), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(169, 177, 214), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special.markdown".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, true)),
        ("repeat".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("string.documentation".into(), Style::new(Color::new(224, 175, 104), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(180, 249, 248), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("symbol".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("tag.delimiter.tsx".into(), Style::new(Color::new(96, 125, 191), None, false, false, false, false)),
        ("tag.tsx".into(), Style::new(Color::new(247, 118, 142), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(36, 40, 59), Some(Color::new(219, 75, 75)), false, false, false, false)),
        ("text.environment".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("text.environment.name".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("text.literal.markdown_inline".into(), Style::new(Color::new(122, 162, 247), Some(Color::new(65, 72, 104)), false, false, false, false)),
        ("text.math".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(26, 188, 156), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(36, 40, 59), Some(Color::new(224, 175, 104)), false, false, false, false)),
        ("text.todo.checked".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("text.todo.unchecked".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(36, 40, 59), Some(Color::new(224, 175, 104)), false, false, false, false)),
        ("type".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(41, 164, 189), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("variable".into(), Style::new(Color::new(192, 202, 245), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(247, 118, 142), None, false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn night() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(192, 202, 245), Some(Color::new(26, 27, 38)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(86, 95, 137), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("constructor.tsx".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("function.call".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("keyword.coroutine".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("label".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("method.call".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(224, 175, 104), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(169, 177, 214), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special.markdown".into(), Style::new(Color::new(255, 158, 100), None, false, false, false, true)),
        ("repeat".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("string.documentation".into(), Style::new(Color::new(224, 175, 104), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(180, 249, 248), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("symbol".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(187, 154, 247), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("tag.delimiter.tsx".into(), Style::new(Color::new(93, 122, 184), None, false, false, false, false)),
        ("tag.tsx".into(), Style::new(Color::new(247, 118, 142), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(26, 27, 38), Some(Color::new(219, 75, 75)), false, false, false, false)),
        ("text.environment".into(), Style::new(Color::new(125, 207, 255), None, false, false, false, false)),
        ("text.environment.name".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(158, 206, 106), None, false, false, false, false)),
        ("text.literal.markdown_inline".into(), Style::new(Color::new(122, 162, 247), Some(Color::new(65, 72, 104)), false, false, false, false)),
        ("text.math".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(26, 188, 156), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(26, 27, 38), Some(Color::new(224, 175, 104)), false, false, false, false)),
        ("text.todo.checked".into(), Style::new(Color::new(115, 218, 202), None, false, false, false, false)),
        ("text.todo.unchecked".into(), Style::new(Color::new(122, 162, 247), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(26, 27, 38), Some(Color::new(224, 175, 104)), false, false, false, false)),
        ("type".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(39, 161, 185), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(42, 195, 222), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(157, 124, 216), None, false, false, true, false)),
        ("variable".into(), Style::new(Color::new(192, 202, 245), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(247, 118, 142), None, false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn day() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(55, 96, 191), Some(Color::new(225, 226, 231)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(88, 117, 57), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(132, 140, 181), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("constructor.tsx".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(56, 112, 104), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("function.call".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(120, 71, 189), None, false, false, true, false)),
        ("keyword.coroutine".into(), Style::new(Color::new(120, 71, 189), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(0, 106, 131), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(120, 71, 189), None, false, false, true, false)),
        ("label".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("method.call".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(0, 106, 131), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(140, 108, 62), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(56, 112, 104), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(97, 114, 176), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(0, 106, 131), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(0, 106, 131), None, false, false, false, false)),
        ("punctuation.special.markdown".into(), Style::new(Color::new(177, 92, 0), None, false, false, false, true)),
        ("repeat".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(88, 117, 57), None, false, false, false, false)),
        ("string.documentation".into(), Style::new(Color::new(140, 108, 62), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(46, 88, 87), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("symbol".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(152, 84, 241), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(56, 112, 104), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("tag.delimiter.tsx".into(), Style::new(Color::new(87, 114, 173), None, false, false, false, false)),
        ("tag.tsx".into(), Style::new(Color::new(245, 42, 101), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(225, 226, 231), Some(Color::new(198, 67, 67)), false, false, false, false)),
        ("text.environment".into(), Style::new(Color::new(0, 113, 151), None, false, false, false, false)),
        ("text.environment.name".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(88, 117, 57), None, false, false, false, false)),
        ("text.literal.markdown_inline".into(), Style::new(Color::new(46, 125, 233), Some(Color::new(161, 166, 197)), false, false, false, false)),
        ("text.math".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(17, 140, 116), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(225, 226, 231), Some(Color::new(140, 108, 62)), false, false, false, false)),
        ("text.todo.checked".into(), Style::new(Color::new(56, 112, 104), None, false, false, false, false)),
        ("text.todo.unchecked".into(), Style::new(Color::new(46, 125, 233), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(225, 226, 231), Some(Color::new(140, 108, 62)), false, false, false, false)),
        ("type".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(36, 150, 172), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(24, 128, 146), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(120, 71, 189), None, false, false, true, false)),
        ("variable".into(), Style::new(Color::new(55, 96, 191), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(245, 42, 101), None, false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn moon() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("_normal".into(), Style::new(Color::new(200, 211, 245), Some(Color::new(34, 36, 54)), false, false, false, false)),
        ("annotation".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("attribute".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("boolean".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, false)),
        ("character".into(), Style::new(Color::new(195, 232, 141), None, false, false, false, false)),
        ("character.special".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("comment".into(), Style::new(Color::new(99, 109, 166), None, false, false, true, false)),
        ("conditional".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("constant".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, false)),
        ("constant.builtin".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("constant.macro".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("constructor".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("constructor.tsx".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("debug".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, false)),
        ("define".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("exception".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("field".into(), Style::new(Color::new(79, 214, 190), None, false, false, false, false)),
        ("float".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, false)),
        ("function".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("function.builtin".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("function.call".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("function.macro".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("include".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("keyword".into(), Style::new(Color::new(252, 167, 234), None, false, false, true, false)),
        ("keyword.coroutine".into(), Style::new(Color::new(252, 167, 234), None, false, false, true, false)),
        ("keyword.function".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("keyword.operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("keyword.return".into(), Style::new(Color::new(252, 167, 234), None, false, false, true, false)),
        ("label".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("macro".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("method".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("method.call".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("namespace".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("number".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, false)),
        ("operator".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("parameter".into(), Style::new(Color::new(255, 199, 119), None, false, false, false, false)),
        ("preproc".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("property".into(), Style::new(Color::new(79, 214, 190), None, false, false, false, false)),
        ("punctuation".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("punctuation.bracket".into(), Style::new(Color::new(130, 139, 184), None, false, false, false, false)),
        ("punctuation.delimiter".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special".into(), Style::new(Color::new(137, 221, 255), None, false, false, false, false)),
        ("punctuation.special.markdown".into(), Style::new(Color::new(255, 150, 108), None, false, false, false, true)),
        ("repeat".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("storageclass".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("string".into(), Style::new(Color::new(195, 232, 141), None, false, false, false, false)),
        ("string.documentation".into(), Style::new(Color::new(255, 199, 119), None, false, false, false, false)),
        ("string.escape".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("string.regex".into(), Style::new(Color::new(180, 249, 248), None, false, false, false, false)),
        ("string.special".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("symbol".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("tag".into(), Style::new(Color::new(192, 153, 255), None, false, false, false, false)),
        ("tag.attribute".into(), Style::new(Color::new(79, 214, 190), None, false, false, false, false)),
        ("tag.delimiter".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("tag.delimiter.tsx".into(), Style::new(Color::new(101, 130, 195), None, false, false, false, false)),
        ("tag.tsx".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
        ("text.danger".into(), Style::new(Color::new(34, 36, 54), Some(Color::new(197, 59, 83)), false, false, false, false)),
        ("text.environment".into(), Style::new(Color::new(134, 225, 252), None, false, false, false, false)),
        ("text.environment.name".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("text.literal".into(), Style::new(Color::new(195, 232, 141), None, false, false, false, false)),
        ("text.literal.markdown_inline".into(), Style::new(Color::new(130, 170, 255), Some(Color::new(68, 74, 115)), false, false, false, false)),
        ("text.math".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("text.note".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("text.reference".into(), Style::new(Color::new(79, 214, 190), None, false, false, false, false)),
        ("text.title".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, true)),
        ("text.todo".into(), Style::new(Color::new(34, 36, 54), Some(Color::new(255, 199, 119)), false, false, false, false)),
        ("text.todo.checked".into(), Style::new(Color::new(79, 214, 190), None, false, false, false, false)),
        ("text.todo.unchecked".into(), Style::new(Color::new(130, 170, 255), None, false, false, false, false)),
        ("text.warning".into(), Style::new(Color::new(34, 36, 54), Some(Color::new(255, 199, 119)), false, false, false, false)),
        ("type".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("type.builtin".into(), Style::new(Color::new(88, 158, 215), None, false, false, false, false)),
        ("type.definition".into(), Style::new(Color::new(101, 188, 255), None, false, false, false, false)),
        ("type.qualifier".into(), Style::new(Color::new(252, 167, 234), None, false, false, true, false)),
        ("variable".into(), Style::new(Color::new(200, 211, 245), None, false, false, false, false)),
        ("variable.builtin".into(), Style::new(Color::new(255, 117, 127), None, false, false, false, false)),
    ]))
}
