//! The onedark themes in this module were extracted from <https://github.com/navarasu/onedark.nvim>
//!
//! The module source is automatically generated with `cargo xtask codegen` inside the
//! syntastica workspace.

use std::collections::BTreeMap;

use syntastica_core::{
    theme,
    theme::{ResolvedTheme, Theme, ThemeValue},
};

pub fn cool() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#242b38",
        "bg1": "#2d3343",
        "bg2": "#343e4f",
        "bg3": "#363c51",
        "bg_blue": "#6db9f7",
        "bg_d": "#1e242e",
        "bg_yellow": "#f0d197",
        "black": "#151820",
        "blue": "#5ab0f6",
        "cyan": "#4dbdcb",
        "dark_cyan": "#25747d",
        "dark_purple": "#8f36a9",
        "dark_red": "#a13131",
        "dark_yellow": "#9a6b16",
        "diff_add": "#303d27",
        "diff_change": "#18344c",
        "diff_delete": "#3c2729",
        "diff_text": "#265478",
        "fg": "#a5b0c5",
        "green": "#97ca72",
        "grey": "#546178",
        "light_grey": "#7d899f",
        "orange": "#d99a5e",
        "purple": "#ca72e4",
        "red": "#ef5f6b",
        "yellow": "#ebc275",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn dark() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#282c34",
        "bg1": "#31353f",
        "bg2": "#393f4a",
        "bg3": "#3b3f4c",
        "bg_blue": "#73b8f1",
        "bg_d": "#21252b",
        "bg_yellow": "#ebd09c",
        "black": "#181a1f",
        "blue": "#61afef",
        "cyan": "#56b6c2",
        "dark_cyan": "#2b6f77",
        "dark_purple": "#8a3fa0",
        "dark_red": "#993939",
        "dark_yellow": "#93691d",
        "diff_add": "#31392b",
        "diff_change": "#1c3448",
        "diff_delete": "#382b2c",
        "diff_text": "#2c5372",
        "fg": "#abb2bf",
        "green": "#98c379",
        "grey": "#5c6370",
        "light_grey": "#848b98",
        "orange": "#d19a66",
        "purple": "#c678dd",
        "red": "#e86671",
        "yellow": "#e5c07b",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn darker() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#1f2329",
        "bg1": "#282c34",
        "bg2": "#30363f",
        "bg3": "#323641",
        "bg_blue": "#61afef",
        "bg_d": "#181b20",
        "bg_yellow": "#e8c88c",
        "black": "#0e1013",
        "blue": "#4fa6ed",
        "cyan": "#48b0bd",
        "dark_cyan": "#266269",
        "dark_purple": "#7e3992",
        "dark_red": "#8b3434",
        "dark_yellow": "#835d1a",
        "diff_add": "#272e23",
        "diff_change": "#172a3a",
        "diff_delete": "#2d2223",
        "diff_text": "#274964",
        "fg": "#a0a8b7",
        "green": "#8ebd6b",
        "grey": "#535965",
        "light_grey": "#7a818e",
        "orange": "#cc9057",
        "purple": "#bf68d9",
        "red": "#e55561",
        "yellow": "#e2b86b",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn deep() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#1a212e",
        "bg1": "#21283b",
        "bg2": "#283347",
        "bg3": "#2a324a",
        "bg_blue": "#54b0fd",
        "bg_d": "#141b24",
        "bg_yellow": "#f2cc81",
        "black": "#0c0e15",
        "blue": "#41a7fc",
        "cyan": "#34bfd0",
        "dark_cyan": "#1b6a73",
        "dark_purple": "#862aa1",
        "dark_red": "#992525",
        "dark_yellow": "#8f610d",
        "diff_add": "#27341c",
        "diff_change": "#102b40",
        "diff_delete": "#331c1e",
        "diff_text": "#1c4a6e",
        "fg": "#93a4c3",
        "green": "#8bcd5b",
        "grey": "#455574",
        "light_grey": "#6c7d9c",
        "orange": "#dd9046",
        "purple": "#c75ae8",
        "red": "#f65866",
        "yellow": "#efbd5d",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn light() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#fafafa",
        "bg1": "#f0f0f0",
        "bg2": "#e6e6e6",
        "bg3": "#dcdcdc",
        "bg_blue": "#68aee8",
        "bg_d": "#c9c9c9",
        "bg_yellow": "#e2c792",
        "black": "#101012",
        "blue": "#4078f2",
        "cyan": "#0184bc",
        "dark_cyan": "#2b5d63",
        "dark_purple": "#79428a",
        "dark_red": "#833b3b",
        "dark_yellow": "#7c5c20",
        "diff_add": "#e2fbe4",
        "diff_change": "#e2ecfb",
        "diff_delete": "#fce2e5",
        "diff_text": "#cad3e0",
        "fg": "#383a42",
        "green": "#50a14f",
        "grey": "#a0a1a7",
        "light_grey": "#818387",
        "orange": "#c18401",
        "purple": "#a626a4",
        "red": "#e45649",
        "yellow": "#986801",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn warm() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#2c2d30",
        "bg1": "#35373b",
        "bg2": "#3e4045",
        "bg3": "#404247",
        "bg_blue": "#79b7eb",
        "bg_d": "#242628",
        "bg_yellow": "#e6cfa1",
        "black": "#191a1c",
        "blue": "#68aee8",
        "cyan": "#5fafb9",
        "dark_cyan": "#316a71",
        "dark_purple": "#854897",
        "dark_red": "#914141",
        "dark_yellow": "#8c6724",
        "diff_add": "#32352f",
        "diff_change": "#203444",
        "diff_delete": "#342f2f",
        "diff_text": "#32526c",
        "fg": "#b1b4b9",
        "green": "#99bc80",
        "grey": "#646568",
        "light_grey": "#8b8d91",
        "orange": "#c99a6e",
        "purple": "#c27fd7",
        "red": "#e16d77",
        "yellow": "#dfbe81",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

pub fn warmer() -> ResolvedTheme {
    let mut palette = theme! {
        "bg0": "#232326",
        "bg1": "#2c2d31",
        "bg2": "#35363b",
        "bg3": "#37383d",
        "bg_blue": "#68aee8",
        "bg_d": "#1b1c1e",
        "bg_yellow": "#e2c792",
        "black": "#101012",
        "blue": "#57a5e5",
        "cyan": "#51a8b3",
        "dark_cyan": "#2b5d63",
        "dark_purple": "#79428a",
        "dark_red": "#833b3b",
        "dark_yellow": "#7c5c20",
        "diff_add": "#282b26",
        "diff_change": "#1a2a37",
        "diff_delete": "#2a2626",
        "diff_text": "#2c485f",
        "fg": "#a7aab0",
        "green": "#8fb573",
        "grey": "#5a5b5e",
        "light_grey": "#818387",
        "orange": "#c49060",
        "purple": "#bb70d2",
        "red": "#de5d68",
        "yellow": "#dbb671",
    }
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}

fn theme() -> BTreeMap<String, ThemeValue> {
    theme! {
        "annotation": "$fg",
        "attribute": "$cyan",
        "boolean": "$orange",
        "character": "$orange",
        "comment": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "grey",
        },
        "conditional": "$purple",
        "constant": "$orange",
        "constant.builtin": "$orange",
        "constant.macro": "$orange",
        "constructor": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "yellow",
        },
        "danger": "$fg",
        "error": "$fg",
        "exception": "$purple",
        "field": "$cyan",
        "float": "$orange",
        "function": "$blue",
        "function.builtin": "$cyan",
        "function.macro": "$cyan",
        "include": "$purple",
        "keyword": "$purple",
        "keyword.function": "$purple",
        "keyword.operator": "$purple",
        "label": "$red",
        "method": "$blue",
        "namespace": "$yellow",
        "none": "$fg",
        "note": "$fg",
        "number": "$orange",
        "operator": "$fg",
        "parameter": "$red",
        "parameter.reference": "$fg",
        "property": "$cyan",
        "punctuation.bracket": "$light_grey",
        "punctuation.delimiter": "$light_grey",
        "punctuation.special": "$red",
        "repeat": "$purple",
        "string": "$green",
        "string.escape": "$red",
        "string.regex": "$orange",
        "symbol": "$cyan",
        "tag": "$purple",
        "tag.attribute": "$yellow",
        "tag.delimiter": "$purple",
        "text": "$fg",
        "text.diff.add": "$green",
        "text.diff.delete": "$red",
        "text.emphasis": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "fg",
        },
        "text.environment": "$fg",
        "text.environment.name": "$fg",
        "text.literal": "$green",
        "text.math": "$fg",
        "text.reference": "$blue",
        "text.strike": {
            color: None,
            underline: false,
            strikethrough: true,
            italic: false,
            bold: false,
            link: "fg",
        },
        "text.strong": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "fg",
        },
        "text.title": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "orange",
        },
        "text.todo": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "red",
        },
        "text.underline": {
            color: None,
            underline: true,
            strikethrough: false,
            italic: false,
            bold: false,
            link: "fg",
        },
        "text.uri": {
            color: None,
            underline: true,
            strikethrough: false,
            italic: false,
            bold: false,
            link: "cyan",
        },
        "type": "$yellow",
        "type.builtin": "$orange",
        "variable": "$fg",
        "variable.builtin": "$red",
        "warning": "$fg",
    }
    .into_inner()
}
