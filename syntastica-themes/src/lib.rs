#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![doc = include_str!("../theme_list.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
// #![deny(missing_docs)]

use syntastica_core::theme::ResolvedTheme;

pub mod abscs;
pub mod aurora;
pub mod blue_moon;
pub mod boo;
pub mod catppuccin;
pub mod darcula;
pub mod dracula;
pub mod everblush;
pub mod everforest;
pub mod falcon;
pub mod github;
pub mod gruvbox;
pub mod material;
pub mod melange;
pub mod minimal;
pub mod monochrome;
pub mod monokai;
pub mod moonfly;
pub mod moonlight;
pub mod neon;
pub mod nightfly;
pub mod nord;
pub mod oceanicnext;
pub mod omni;
pub mod one;
pub mod oxocarbon;
pub mod solarized;
pub mod tokyo;
pub mod vscode;
pub mod zephyr;

/////////////////////////////////////////////
//// All following code is autogenerated ////
//// by running `cargo xtask codegen` in ////
//// the syntastica workspace. //////////////
/////////////////////////////////////////////

/// Try to get a theme given its path as a string.
///
/// For a list of all acceptable theme names see [`THEMES`].
///
/// # Example
///
/// ```
/// assert_eq!(
///     syntastica_themes::from_str("one::dark"),
///     Some(syntastica_themes::one::dark()),
/// );
/// ```
pub fn from_str(theme_name: impl AsRef<str>) -> Option<ResolvedTheme> {
    match theme_name.as_ref() {
        "abscs::abscs" => Some(abscs::abscs()),
        "aurora::aurora" => Some(aurora::aurora()),
        "blue_moon::blue_moon" => Some(blue_moon::blue_moon()),
        "boo::boo" => Some(boo::boo()),
        "catppuccin::frappe" => Some(catppuccin::frappe()),
        "catppuccin::latte" => Some(catppuccin::latte()),
        "catppuccin::macchiato" => Some(catppuccin::macchiato()),
        "catppuccin::mocha" => Some(catppuccin::mocha()),
        "darcula::darcula" => Some(darcula::darcula()),
        "dracula::dracula" => Some(dracula::dracula()),
        "everblush::everblush" => Some(everblush::everblush()),
        "everforest::dark" => Some(everforest::dark()),
        "everforest::light" => Some(everforest::light()),
        "falcon::falcon" => Some(falcon::falcon()),
        "github::dark" => Some(github::dark()),
        "github::dark_colorblind" => Some(github::dark_colorblind()),
        "github::dark_default" => Some(github::dark_default()),
        "github::dark_dimmed" => Some(github::dark_dimmed()),
        "github::dark_high_contrast" => Some(github::dark_high_contrast()),
        "github::dark_tritanopia" => Some(github::dark_tritanopia()),
        "github::dimmed" => Some(github::dimmed()),
        "github::light" => Some(github::light()),
        "github::light_colorblind" => Some(github::light_colorblind()),
        "github::light_default" => Some(github::light_default()),
        "github::light_high_contrast" => Some(github::light_high_contrast()),
        "github::light_tritanopia" => Some(github::light_tritanopia()),
        "gruvbox::dark" => Some(gruvbox::dark()),
        "gruvbox::light" => Some(gruvbox::light()),
        "material::darker" => Some(material::darker()),
        "material::deep_ocean" => Some(material::deep_ocean()),
        "material::lighter" => Some(material::lighter()),
        "material::oceanic" => Some(material::oceanic()),
        "material::palenight" => Some(material::palenight()),
        "melange::melange" => Some(melange::melange()),
        "minimal::minimal" => Some(minimal::minimal()),
        "monochrome::monochrome" => Some(monochrome::monochrome()),
        "monokai::monokai" => Some(monokai::monokai()),
        "monokai::pro" => Some(monokai::pro()),
        "monokai::ristretto" => Some(monokai::ristretto()),
        "monokai::soda" => Some(monokai::soda()),
        "moonfly::moonfly" => Some(moonfly::moonfly()),
        "moonlight::moonlight" => Some(moonlight::moonlight()),
        "neon::dark" => Some(neon::dark()),
        "neon::default" => Some(neon::default()),
        "neon::doom" => Some(neon::doom()),
        "neon::light" => Some(neon::light()),
        "nightfly::nightfly" => Some(nightfly::nightfly()),
        "nord::nord" => Some(nord::nord()),
        "oceanicnext::dark" => Some(oceanicnext::dark()),
        "oceanicnext::light" => Some(oceanicnext::light()),
        "omni::omni" => Some(omni::omni()),
        "one::cool" => Some(one::cool()),
        "one::dark" => Some(one::dark()),
        "one::darker" => Some(one::darker()),
        "one::deep" => Some(one::deep()),
        "one::light" => Some(one::light()),
        "one::warm" => Some(one::warm()),
        "one::warmer" => Some(one::warmer()),
        "oxocarbon::dark" => Some(oxocarbon::dark()),
        "oxocarbon::light" => Some(oxocarbon::light()),
        "solarized::dark" => Some(solarized::dark()),
        "solarized::light" => Some(solarized::light()),
        "tokyo::day" => Some(tokyo::day()),
        "tokyo::moon" => Some(tokyo::moon()),
        "tokyo::night" => Some(tokyo::night()),
        "tokyo::storm" => Some(tokyo::storm()),
        "vscode::dark" => Some(vscode::dark()),
        "vscode::light" => Some(vscode::light()),
        "zephyr::zephyr" => Some(zephyr::zephyr()),
        _ => None,
    }
}

/// A list of all theme names as they are accepted by [`from_str`].
pub const THEMES: &[&str] = &[
    "abscs::abscs",
    "aurora::aurora",
    "blue_moon::blue_moon",
    "boo::boo",
    "catppuccin::frappe",
    "catppuccin::latte",
    "catppuccin::macchiato",
    "catppuccin::mocha",
    "darcula::darcula",
    "dracula::dracula",
    "everblush::everblush",
    "everforest::dark",
    "everforest::light",
    "falcon::falcon",
    "github::dark",
    "github::dark_colorblind",
    "github::dark_default",
    "github::dark_dimmed",
    "github::dark_high_contrast",
    "github::dark_tritanopia",
    "github::dimmed",
    "github::light",
    "github::light_colorblind",
    "github::light_default",
    "github::light_high_contrast",
    "github::light_tritanopia",
    "gruvbox::dark",
    "gruvbox::light",
    "material::darker",
    "material::deep_ocean",
    "material::lighter",
    "material::oceanic",
    "material::palenight",
    "melange::melange",
    "minimal::minimal",
    "monochrome::monochrome",
    "monokai::monokai",
    "monokai::pro",
    "monokai::ristretto",
    "monokai::soda",
    "moonfly::moonfly",
    "moonlight::moonlight",
    "neon::dark",
    "neon::default",
    "neon::doom",
    "neon::light",
    "nightfly::nightfly",
    "nord::nord",
    "oceanicnext::dark",
    "oceanicnext::light",
    "omni::omni",
    "one::cool",
    "one::dark",
    "one::darker",
    "one::deep",
    "one::light",
    "one::warm",
    "one::warmer",
    "oxocarbon::dark",
    "oxocarbon::light",
    "solarized::dark",
    "solarized::light",
    "tokyo::day",
    "tokyo::moon",
    "tokyo::night",
    "tokyo::storm",
    "vscode::dark",
    "vscode::light",
    "zephyr::zephyr",
];
