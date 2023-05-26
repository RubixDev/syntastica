use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use fancy_regex::Regex;
use syntastica_core::theme::ThemeValue;

use crate::codegen;

pub fn make_theme() -> Result<String> {
    let colors_map = HashMap::from([
        ("Fg", "fg"),
        ("LightGrey", "light_grey"),
        ("Grey", "grey"),
        ("Red", "red"),
        ("Cyan", "cyan"),
        ("Yellow", "yellow"),
        ("Orange", "orange"),
        ("Green", "green"),
        ("Blue", "blue"),
        ("Purple", "purple"),
    ]);

    let code_style = HashMap::from([
        ("comments", Some("italic")),
        ("keywords", None),
        ("functions", None),
        ("strings", None),
        ("variables", None),
    ]);

    let groups_regex = Regex::new(r#"\["@([\w.]+)"\]\s*=\s*(?:colors\.(\w+)|\{fg\s*=\s*c\.(\w+),\s*fmt\s*=\s*(?:cfg.code_style.(\w+)|(["'])(.*)\5)\})"#).unwrap();
    let colors_regex = Regex::new(r#"(\w+) = "(#[a-fA-F0-9]{6})","#).unwrap();

    //// fetch source files ////

    println!("Fetching `groups.lua`");
    let groups_lua = reqwest::blocking::get(
        "https://raw.githubusercontent.com/navarasu/onedark.nvim/master/lua/onedark/highlights.lua",
    )?
    .text()?;
    println!("Fetching `palette.lua`");
    let palette_lua = reqwest::blocking::get(
        "https://raw.githubusercontent.com/navarasu/onedark.nvim/master/lua/onedark/palette.lua",
    )?
    .text()?;
    println!("Fetching complete");

    //// parse color palettes ////

    const VARIANTS: &[&str] = &["dark", "darker", "cool", "deep", "warm", "warmer", "light"];
    let mut palettes = BTreeMap::new();
    for variant in VARIANTS {
        let mut palette = BTreeMap::new();
        for match_ in colors_regex.captures_iter(
            palette_lua
                .split_once(&format!("{variant} = {{"))
                .unwrap()
                .1
                .split_once('}')
                .unwrap()
                .0,
        ) {
            let match_ = match_?;
            palette.insert(
                match_[1].to_owned(),
                ThemeValue::Simple(match_[2].to_owned()),
            );
        }
        palettes.insert(variant, palette);
    }

    //// parse theme ////
    let mut theme = BTreeMap::new();
    for match_ in groups_regex.captures_iter(&groups_lua) {
        let match_ = match_?;
        if let Some(color) = match_.get(2) {
            theme.insert(
                match_[1].to_owned(),
                ThemeValue::Simple(format!("${}", &colors_map[color.as_str()])),
            );
        } else {
            let color = None;
            let link = Some(match_[3].to_owned());
            let mut underline = false;
            let mut strikethrough = false;
            let mut italic = false;
            let mut bold = false;
            match match_
                .get(4)
                .and_then(|key| code_style[key.as_str()])
                .or_else(|| match_.get(6).map(|s| s.as_str()))
            {
                Some("underline") => underline = true,
                Some("strikethrough") => strikethrough = true,
                Some("italic") => italic = true,
                Some("bold") => bold = true,
                Some(_) | None => {
                    theme.insert(
                        match_[1].to_owned(),
                        ThemeValue::Simple(format!("${}", &match_[3])),
                    );
                    continue;
                }
            }

            theme.insert(
                match_[1].to_owned(),
                ThemeValue::Extended {
                    color,
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link,
                },
            );
        }
    }

    //// construct the entire module file ////

    Ok(codegen::make_theme_file(
        "onedark",
        "https://github.com/navarasu/onedark.nvim",
        palettes,
        theme,
    ))
}
