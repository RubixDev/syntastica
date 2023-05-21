use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use fancy_regex::Regex;
use syntastica::config::ThemeValue;

use crate::codegen::{self, RawThemeValue};

pub fn make_theme() -> Result<String> {
    let config = HashMap::from([
        ("undercurl", true),
        ("underline", true),
        ("bold", true),
        ("italic.strings", true),
        ("italic.comments", true),
        ("italic.operators", false),
        ("italic.folds", true),
        ("strikethrough", true),
        ("invert_selection", false),
        ("invert_signs", false),
        ("invert_tabline", false),
        ("invert_intend_guides", false),
        ("inverse", true),
        ("dim_inactive", false),
        ("transparent_mode", false),
    ]);

    let resolve_from_config = |raw: Option<&String>| -> bool {
        match raw {
            Some(raw) => match raw.strip_prefix("config.") {
                Some(key) => config[key],
                None => raw == "true",
            },
            None => false,
        }
    };

    let groups_regex = Regex::new(r#"(?:(\w+)|\["(@[\w.]+)"\])\s*=\s*\{\s*(?:link\s*=\s*(["])(.*?)\3|((?:\w+ = [^,]+?(?:,\s*)?)+))\s*\}"#).unwrap();
    let colors_regex = Regex::new(r#"(\w+) = "(#[a-fA-F0-9]{6})","#).unwrap();
    let palette_regex = Regex::new(r#"(\w+) = p\.(\w+),"#).unwrap();

    //// fetch source files ////

    println!("Fetching `groups.lua`");
    let groups_lua = reqwest::blocking::get(
        "https://raw.githubusercontent.com/ellisonleao/gruvbox.nvim/main/lua/gruvbox/groups.lua",
    )?
    .text()?;
    println!("Fetching `palette.lua`");
    let palette_lua = reqwest::blocking::get(
        "https://raw.githubusercontent.com/ellisonleao/gruvbox.nvim/main/lua/gruvbox/palette.lua",
    )?
    .text()?;
    println!("Fetching complete");

    //// parse color palettes ////

    let raw_colors = colors_regex
        .captures_iter(
            palette_lua
                .split_once("M.colors = {")
                .unwrap()
                .1
                .split_once('}')
                .unwrap()
                .0,
        )
        .map(|match_| {
            match_.map(|match_| {
                (
                    match_.get(1).unwrap().as_str(),
                    match_.get(2).unwrap().as_str(),
                )
            })
        })
        .collect::<Result<HashMap<_, _>, _>>()?;
    const VARIANTS: &[&str] = &["dark", "light"];
    let mut palettes = BTreeMap::new();
    for variant in VARIANTS {
        let mut palette = BTreeMap::new();
        for match_ in palette_regex.captures_iter(
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
                ThemeValue::Simple(raw_colors[&match_[2]].to_owned()),
            );
        }
        palettes.insert(variant, palette);
    }

    //// parse theme ////
    let mut raw_theme = BTreeMap::new();
    for match_ in groups_regex.captures_iter(&groups_lua) {
        let match_ = match_?;
        let Some(key) = match_.get(1).or_else(|| match_.get(2)) else { continue; };
        let value = match_.get(4).map_or_else(
            || {
                RawThemeValue::Styles(
                    match_[5]
                        .split(',')
                        .filter_map(|item| item.split_once('='))
                        .map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned()))
                        .filter(|(k, _)| {
                            ["fg", "italic", "bold", "underline", "strikethrough"]
                                .contains(&k.as_str())
                        })
                        .collect(),
                )
            },
            |link| RawThemeValue::Link(link.as_str().to_owned()),
        );
        raw_theme.insert(key.as_str().to_owned(), value);
    }
    // resolve links in raw_theme
    codegen::resolve_links(&mut raw_theme);
    // construct theme map
    let mut theme = BTreeMap::new();
    for (group, styling) in raw_theme {
        if !group.starts_with('@') || group.starts_with("@lsp") {
            continue;
        }
        let key = group[1..].to_owned();
        let styling = match styling {
            RawThemeValue::Link(_) => panic!("links were resolved"),
            RawThemeValue::Styles(map) => map,
            RawThemeValue::Ignore => continue,
        };

        if styling.len() == 1 && styling.contains_key("fg") {
            let raw = &styling["fg"];
            if let Some(name) = raw.strip_prefix("colors.") {
                // link to color by name
                theme.insert(key, ThemeValue::Simple(format!("${name}")));
            } else {
                // strip quotes
                let color = &raw[1..raw.len() - 1];
                if color.starts_with('#') && color.len() == 7 {
                    theme.insert(key, ThemeValue::Simple(color.to_owned()));
                } else {
                    eprintln!("warning: ignoring invalid color '{color}'");
                }
            }
        } else {
            let mut color = None;
            let mut link = None;
            if let Some(raw) = styling.get("fg") {
                if let Some(name) = raw.strip_prefix("colors.") {
                    // link to color by name
                    link = Some(name.to_owned());
                } else {
                    // strip quotes and use value
                    let color_str = &raw[1..raw.len() - 1];
                    if color_str.starts_with('#') && color_str.len() == 7 {
                        color = Some(color_str.to_owned());
                    } else {
                        eprintln!("warning: ignoring invalid color '{color_str}'");
                    }
                }
            }
            if color.is_none() && link.is_none() {
                link = Some("fg1".to_string());
            }

            let underline = resolve_from_config(styling.get("underline"));
            let strikethrough = resolve_from_config(styling.get("strikethrough"));
            let italic = resolve_from_config(styling.get("italic"));
            let bold = resolve_from_config(styling.get("bold"));

            theme.insert(
                key,
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

    //// construct and return the entire module file ////

    Ok(codegen::make_theme_file(
        "gruvbox",
        "https://github.com/ellisonleao/gruvbox.nvim",
        palettes,
        theme,
    ))
}
