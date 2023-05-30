/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. If `undefined`, all languages will be loaded.
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
 */
export declare function highlight(code: string, language: string, theme: string): string;
