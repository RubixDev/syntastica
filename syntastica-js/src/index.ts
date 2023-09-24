import initModule, { type SyntasticaModule } from '../pkg/syntastica-js.js'

const PTR_SIZE = Float32Array.BYTES_PER_ELEMENT

/**
 * A theme name to pass to {@link highlight} or {@link render}.
 */
export type Theme = typeof THEMES[number]

/**
 * A language name to pass to {@link init}, {@link highlight}, or {@link process}.
 */
export type Language = typeof LANGUAGES[number]

let Module: SyntasticaModule = null as unknown as SyntasticaModule

/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. By default, all languages will be loaded.
 * See [here](https://rubixdev.github.io/syntastica/syntastica_parsers_git/) for a list of supported languages.
 */
export async function init(languages?: Language[]) {
    if (Module === null) {
        Module = await initModule()
    }

    if (languages === undefined) {
        Module._init(0, 0)
    } else {
        // allocate an array
        const list_len = languages.length
        const list_ptr = Module._malloc(list_len * PTR_SIZE)

        // store pointers to the string in the array
        for (let i = 0; i < list_len; i++) {
            Module.setValue(list_ptr + i * PTR_SIZE, Module.stringToNewUTF8(languages[i]), '*')
        }

        // call `init`
        Module._init(list_ptr, list_len)

        // free everything
        for (let i = 0; i < list_len; i++) {
            Module._free(Module.getValue(list_ptr + i * PTR_SIZE, '*'))
        }
        Module._free(list_ptr)
    }
}

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
export function highlight(code: string, language: Language, theme: Theme, renderer: string = 'HTML'): string {
    const code_ptr = Module.stringToNewUTF8(code)
    const language_ptr = Module.stringToNewUTF8(language)
    const theme_ptr = Module.stringToNewUTF8(theme)
    const renderer_ptr = Module.stringToNewUTF8(renderer)

    const result_ptr = Module._highlight(code_ptr, language_ptr, theme_ptr, renderer_ptr)
    const result = Module.UTF8ToString(result_ptr)

    Module._free(code_ptr)
    Module._free(language_ptr)
    Module._free(theme_ptr)
    Module._free(renderer_ptr)
    Module._free(result_ptr)

    return result
}

/**
 * Prepare code for rendering multiple times.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link init}.
 */
export function process(code: string, language: Language) {
    const code_ptr = Module.stringToNewUTF8(code)
    const language_ptr = Module.stringToNewUTF8(language)

    Module._process(code_ptr, language_ptr)

    Module._free(code_ptr)
    Module._free(language_ptr)
}

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
export function render(theme: Theme, renderer: string = 'HTML'): string {
    const theme_ptr = Module.stringToNewUTF8(theme)
    const renderer_ptr = Module.stringToNewUTF8(renderer)

    const result_ptr = Module._render(theme_ptr, renderer_ptr)
    const result = Module.UTF8ToString(result_ptr)

    Module._free(theme_ptr)
    Module._free(renderer_ptr)
    Module._free(result_ptr)

    return result
}

export default { init, highlight, process, render }

// DISCLAIMER: All code below this line is generated with `cargo xtask codegen js-list`
// in the syntastica workspace. Do not edit this code manually!
/**
 * A list of all supported languages.
 *
 * @see The {@link Language} type.
 */
export const LANGUAGES = [
    'asm',
    'bash',
    'c',
    'c_sharp',
    'comment',
    'cpp',
    'css',
    'dart',
    'diff',
    'ebnf',
    'ejs',
    'erb',
    'go',
    'haskell',
    'hexdump',
    'html',
    'java',
    'javascript',
    'jsdoc',
    'json',
    'json5',
    'jsonc',
    'julia',
    'latex',
    'llvm',
    'lua',
    'markdown',
    'markdown_inline',
    'ocaml',
    'ocaml_interface',
    'php',
    'python',
    'ql',
    'regex',
    'ruby',
    'rush',
    'rust',
    'scala',
    'scss',
    'toml',
    'tsx',
    'typescript',
    'ursa',
    'verilog',
    'wat',
    'yaml',
] as const

/**
 * A list of all valid themes.
 *
 * @see The {@link Theme} type.
 */
export const THEMES = [
    'abscs::abscs',
    'aurora::aurora',
    'blue_moon::blue_moon',
    'boo::boo',
    'catppuccin::frappe',
    'catppuccin::latte',
    'catppuccin::macchiato',
    'catppuccin::mocha',
    'darcula::darcula',
    'dracula::dracula',
    'everblush::everblush',
    'everforest::dark',
    'everforest::light',
    'falcon::falcon',
    'github::dark',
    'github::dark_colorblind',
    'github::dark_default',
    'github::dark_dimmed',
    'github::dark_high_contrast',
    'github::dark_tritanopia',
    'github::dimmed',
    'github::light',
    'github::light_colorblind',
    'github::light_default',
    'github::light_high_contrast',
    'github::light_tritanopia',
    'gruvbox::dark',
    'gruvbox::light',
    'material::darker',
    'material::deep_ocean',
    'material::lighter',
    'material::oceanic',
    'material::palenight',
    'melange::melange',
    'minimal::minimal',
    'monochrome::monochrome',
    'monokai::monokai',
    'monokai::pro',
    'monokai::ristretto',
    'monokai::soda',
    'moonfly::moonfly',
    'moonlight::moonlight',
    'neon::dark',
    'neon::default',
    'neon::doom',
    'neon::light',
    'nightfly::nightfly',
    'nord::nord',
    'oceanicnext::dark',
    'oceanicnext::light',
    'omni::omni',
    'one::cool',
    'one::dark',
    'one::darker',
    'one::deep',
    'one::light',
    'one::warm',
    'one::warmer',
    'oxocarbon::dark',
    'oxocarbon::light',
    'solarized::dark',
    'solarized::light',
    'tokyo::day',
    'tokyo::moon',
    'tokyo::night',
    'tokyo::storm',
    'vscode::dark',
    'vscode::light',
    'zephyr::zephyr',
] as const
