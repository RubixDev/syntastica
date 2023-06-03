//! The tokyo theme collection in this module was extracted from <https://github.com/folke/tokyonight.nvim> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn storm() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(36, 40, 59)),
        ("annotation".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(86, 95, 137), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("constructor.tsx".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("keyword.coroutine".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("method.call".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(224, 175, 104), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(169, 177, 214), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special.markdown".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, true)),
        ("repeat".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("string.documentation".to_owned(), Style::new(Color::new(224, 175, 104), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(180, 249, 248), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("tag.delimiter.tsx".to_owned(), Style::new(Color::new(96, 125, 191), false, false, false, false)),
        ("tag.tsx".to_owned(), Style::new(Color::new(247, 118, 142), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(36, 40, 59), false, false, false, false)),
        ("text.environment".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("text.math".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(26, 188, 156), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(36, 40, 59), false, false, false, false)),
        ("text.todo.checked".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("text.todo.unchecked".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(36, 40, 59), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(41, 164, 189), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("variable".to_owned(), Style::new(Color::new(192, 202, 245), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(247, 118, 142), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn night() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(26, 27, 38)),
        ("annotation".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(86, 95, 137), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("constructor.tsx".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("keyword.coroutine".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("method.call".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(224, 175, 104), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(169, 177, 214), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special.markdown".to_owned(), Style::new(Color::new(255, 158, 100), false, false, false, true)),
        ("repeat".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("string.documentation".to_owned(), Style::new(Color::new(224, 175, 104), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(180, 249, 248), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(187, 154, 247), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("tag.delimiter.tsx".to_owned(), Style::new(Color::new(93, 122, 184), false, false, false, false)),
        ("tag.tsx".to_owned(), Style::new(Color::new(247, 118, 142), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(26, 27, 38), false, false, false, false)),
        ("text.environment".to_owned(), Style::new(Color::new(125, 207, 255), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(158, 206, 106), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("text.math".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(26, 188, 156), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(26, 27, 38), false, false, false, false)),
        ("text.todo.checked".to_owned(), Style::new(Color::new(115, 218, 202), false, false, false, false)),
        ("text.todo.unchecked".to_owned(), Style::new(Color::new(122, 162, 247), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(26, 27, 38), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(39, 161, 185), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(42, 195, 222), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(157, 124, 216), false, false, true, false)),
        ("variable".to_owned(), Style::new(Color::new(192, 202, 245), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(247, 118, 142), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn day() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(225, 226, 231)),
        ("annotation".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(88, 117, 57), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(132, 140, 181), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("constructor.tsx".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(56, 112, 104), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(120, 71, 189), false, false, true, false)),
        ("keyword.coroutine".to_owned(), Style::new(Color::new(120, 71, 189), false, false, true, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(0, 106, 131), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(120, 71, 189), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("method.call".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(0, 106, 131), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(140, 108, 62), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(56, 112, 104), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(97, 114, 176), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(0, 106, 131), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(0, 106, 131), false, false, false, false)),
        ("punctuation.special.markdown".to_owned(), Style::new(Color::new(177, 92, 0), false, false, false, true)),
        ("repeat".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(88, 117, 57), false, false, false, false)),
        ("string.documentation".to_owned(), Style::new(Color::new(140, 108, 62), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(46, 88, 87), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(152, 84, 241), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(56, 112, 104), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("tag.delimiter.tsx".to_owned(), Style::new(Color::new(87, 114, 173), false, false, false, false)),
        ("tag.tsx".to_owned(), Style::new(Color::new(245, 42, 101), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(225, 226, 231), false, false, false, false)),
        ("text.environment".to_owned(), Style::new(Color::new(0, 113, 151), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(88, 117, 57), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("text.math".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(17, 140, 116), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(225, 226, 231), false, false, false, false)),
        ("text.todo.checked".to_owned(), Style::new(Color::new(56, 112, 104), false, false, false, false)),
        ("text.todo.unchecked".to_owned(), Style::new(Color::new(46, 125, 233), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(225, 226, 231), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(36, 150, 172), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(24, 128, 146), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(120, 71, 189), false, false, true, false)),
        ("variable".to_owned(), Style::new(Color::new(55, 96, 191), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(245, 42, 101), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn moon() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(34, 36, 54)),
        ("annotation".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("attribute".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("boolean".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(195, 232, 141), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(99, 109, 166), false, false, true, false)),
        ("conditional".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("constructor.tsx".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(79, 214, 190), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("function.call".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(252, 167, 234), false, false, true, false)),
        ("keyword.coroutine".to_owned(), Style::new(Color::new(252, 167, 234), false, false, true, false)),
        ("keyword.function".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("keyword.return".to_owned(), Style::new(Color::new(252, 167, 234), false, false, true, false)),
        ("label".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("method.call".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(255, 199, 119), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(79, 214, 190), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(130, 139, 184), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(137, 221, 255), false, false, false, false)),
        ("punctuation.special.markdown".to_owned(), Style::new(Color::new(255, 150, 108), false, false, false, true)),
        ("repeat".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(195, 232, 141), false, false, false, false)),
        ("string.documentation".to_owned(), Style::new(Color::new(255, 199, 119), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("string.regex".to_owned(), Style::new(Color::new(180, 249, 248), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(192, 153, 255), false, false, false, false)),
        ("tag.attribute".to_owned(), Style::new(Color::new(79, 214, 190), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("tag.delimiter.tsx".to_owned(), Style::new(Color::new(101, 130, 195), false, false, false, false)),
        ("tag.tsx".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
        ("text.danger".to_owned(), Style::new(Color::new(34, 36, 54), false, false, false, false)),
        ("text.environment".to_owned(), Style::new(Color::new(134, 225, 252), false, false, false, false)),
        ("text.environment.name".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(195, 232, 141), false, false, false, false)),
        ("text.literal.markdown_inline".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("text.math".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("text.note".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(79, 214, 190), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(34, 36, 54), false, false, false, false)),
        ("text.todo.checked".to_owned(), Style::new(Color::new(79, 214, 190), false, false, false, false)),
        ("text.todo.unchecked".to_owned(), Style::new(Color::new(130, 170, 255), false, false, false, false)),
        ("text.warning".to_owned(), Style::new(Color::new(34, 36, 54), false, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("type.builtin".to_owned(), Style::new(Color::new(88, 158, 215), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(101, 188, 255), false, false, false, false)),
        ("type.qualifier".to_owned(), Style::new(Color::new(252, 167, 234), false, false, true, false)),
        ("variable".to_owned(), Style::new(Color::new(200, 211, 245), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(255, 117, 127), false, false, false, false)),
    ]))
}
