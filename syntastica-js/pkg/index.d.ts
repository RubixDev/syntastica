/**
 * A theme to pass to {@link highlight} or {@link render}.
 */
export type Theme = 'one::dark' | 'one::darker' | 'one::cool' | 'one::deep' | 'one::warm' | 'one::warmer' | 'one::light' | 'gruvbox::dark' | 'gruvbox::light';
/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. By default, all languages will be loaded.
 * See [here](https://rubixdev.github.io/syntastica/syntastica_parsers_git/) for a list of supported languages.
 */
export declare function init(languages?: string[]): Promise<void>;
/**
 * Highlight code and render to HTML.
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
 * @returns The highlighted code as HTML code.
 *
 * See {@link https://rubixdev.github.io/syntastica-ci-test/syntastica/renderer/struct.HtmlRenderer.html | here} for
 * more information on the output.
 */
export declare function highlight(code: string, language: string, theme: Theme): string;
/**
 * Prepare code for rendering multiple times.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 */
export declare function process(code: string, language: string): void;
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
