/**
 * A theme name to pass to {@link highlight} or {@link render}.
 */
export type Theme = typeof THEMES[number];
/**
 * A language name to pass to {@link init}, {@link highlight}, or {@link process}.
 */
export type Language = typeof LANGUAGES[number];
/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. By default, all languages will be loaded.
 * See [here](https://rubixdev.github.io/syntastica/syntastica_parsers_git/) for a list of supported languages.
 */
export declare function init(languages?: Language[]): Promise<void>;
/**
 * Highlight code and render to the requested format.
 *
 * If you plan to highlight the same input multiple times, use {@link process} and {@link render} instead.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`.
 *
 * @param renderer - The renderer to use.
 *
 * The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
 * for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
 *
 * By default, the `HTML` renderer will be used.
 *
 * @returns The highlighted code as HTML code.
 *
 * See {@link https://rubixdev.github.io/syntastica-ci-test/syntastica/renderer/struct.HtmlRenderer.html | here} for
 * more information on the output.
 */
export declare function highlight(code: string, language: Language, theme: Theme, renderer?: string): string;
/**
 * Prepare code for rendering multiple times.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 */
export declare function process(code: string, language: Language): void;
/**
 * Render code that was previously processed by calling {@link process}.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`.
 *
 * @param renderer - The renderer to use.
 *
 * The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
 * for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
 *
 * By default, the `HTML` renderer will be used.
 *
 * @returns The highlighted code in the requested format.
 */
export declare function render(theme: Theme, renderer?: string): string;
declare const _default: {
    init: typeof init;
    highlight: typeof highlight;
    process: typeof process;
    render: typeof render;
};
export default _default;
/**
 * A list of all supported languages.
 *
 * @see The {@link Language} type.
 */
export declare const LANGUAGES: readonly ["asm", "bash", "c", "c_sharp", "cpp", "css", "ejs", "erb", "go", "haskell", "hexdump", "html", "java", "javascript", "jsdoc", "json", "ocaml", "ocaml_interface", "php", "python", "ql", "regex", "ruby", "rush", "rust", "scala", "tsx", "typescript", "verilog", "wat"];
/**
 * A list of all valid themes.
 *
 * @see The {@link Theme} type.
 */
export declare const THEMES: readonly ["abscs::abscs", "aurora::aurora", "blue_moon::blue_moon", "boo::boo", "catppuccin::frappe", "catppuccin::latte", "catppuccin::macchiato", "catppuccin::mocha", "darcula::darcula", "dracula::dracula", "everblush::everblush", "everforest::dark", "everforest::light", "falcon::falcon", "github::dark", "github::dark_colorblind", "github::dark_default", "github::dark_dimmed", "github::dark_high_contrast", "github::dark_tritanopia", "github::dimmed", "github::light", "github::light_colorblind", "github::light_default", "github::light_high_contrast", "github::light_tritanopia", "gruvbox::dark", "gruvbox::light", "material::darker", "material::deep_ocean", "material::lighter", "material::oceanic", "material::palenight", "melange::melange", "minimal::minimal", "monochrome::monochrome", "monokai::monokai", "monokai::pro", "monokai::ristretto", "monokai::soda", "moonfly::moonfly", "moonlight::moonlight", "neon::dark", "neon::default", "neon::doom", "neon::light", "nightfly::nightfly", "nord::nord", "oceanicnext::dark", "oceanicnext::light", "omni::omni", "one::cool", "one::dark", "one::darker", "one::deep", "one::light", "one::warm", "one::warmer", "oxocarbon::dark", "oxocarbon::light", "solarized::dark", "solarized::light", "tokyo::day", "tokyo::moon", "tokyo::night", "tokyo::storm", "vscode::dark", "vscode::light", "zephyr::zephyr"];
