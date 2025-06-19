import initModule, { type SyntasticaModule } from './syntastica-js.js'

const PTR_SIZE = Float32Array.BYTES_PER_ELEMENT

let Module: SyntasticaModule = null as unknown as SyntasticaModule

/**
 * A theme name to pass to {@link highlight} or {@link render}.
 */
export type BuiltinTheme = typeof BUILTIN_THEMES[number]

/**
 * Source code with theme-independent style information attached.
 *
 * See {@link https://rubixdev.github.io/syntastica/syntastica/type.Highlights.html | the Rust definition}
 * for more information.
 */
export type Highlights = [string, string | null][][]

/**
 * A non-transparent color with red, green, and blue values between 0 and 255.
 *
 * Mirrors {@link https://rubixdev.github.io/syntastica/syntastica/style/type.Color.html | this Rust definition}.
 */
export interface Color {
    red: number
    green: number
    blue: number
}

/**
 * Defines how to style a region of text.
 *
 * Besides a main foreground {@link Color}, an optional background color and the following four booleans can be set:
 *
 * - underline
 * - strikethrough
 * - italic
 * - bold
 *
 * Mirrors {@link https://rubixdev.github.io/syntastica/syntastica/style/struct.Style.html | this Rust definition}.
 */
export interface Style {
    color: Color
    bg: Color | null
    underline: boolean
    strikethrough: boolean
    italic: boolean
    bold: boolean
}

/**
 * Defines how to style highlight captures.
 *
 * Mirrors {@link https://rubixdev.github.io/syntastica/syntastica/theme/struct.ResolvedTheme.html | this Rust definition}.
 */
export class Theme {
    constructor(public styles: Record<string, Style>) {}

    /**
     * The default foreground color, if the theme defines one.
     */
    get fg(): Color | null {
        return this.styles['_normal']?.color || null
    }

    /**
     * The default background color, if the theme defines one.
     */
    get bg(): Color | null {
        return this.styles['_normal']?.bg || null
    }

    /**
     * Retreives the style map for a {@link BuiltinTheme}.
     */
    static fromBuiltin(theme: BuiltinTheme): Theme {
        checkModule()
        const errmsgPtr = Module._malloc(PTR_SIZE)
        const themePtr = Module.stringToNewUTF8(theme)

        const resultPtr = Module._get_builtin_theme(errmsgPtr, themePtr)
        if (resultPtr === 0) {
            const errPtr = Module.getValue(errmsgPtr, 'i8*')
            const err = Module.UTF8ToString(errPtr)
            Module._free(errPtr)
            Module._free(errmsgPtr)
            Module._free(themePtr)
            Module._free(resultPtr)
            throw new Error(err)
        }
        const result = JSON.parse(Module.UTF8ToString(resultPtr)) as Record<string, Style>

        Module._free(errmsgPtr)
        Module._free(themePtr)
        Module._free(resultPtr)
        return new Theme(result)
    }
}

/**
 * Initialize the Wasm module.
 *
 * This function _must_ be called before any of the others
 */
export async function init(moduleOverrides?: any) {
    if (Module === null) {
        Module = await initModule(moduleOverrides)
    }
}

function checkModule() {
    if (Module === null) throw new Error('syntastica module is not initialized, call `init` first')
}

/**
 * Load a language from a WebAssembly module.
 *
 * The module can be provided as a path to a file or as a buffer.
 */
export async function loadLanguage(input: string | Uint8Array): Promise<void> {
    checkModule()
    let bytes: Promise<Uint8Array>
    if (input instanceof Uint8Array) {
        bytes = Promise.resolve(input)
    } else {
        // @ts-expect-error
        // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
        if (globalThis.process?.versions.node) {
            // @ts-expect-error
            // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-require-imports
            const fs: typeof import('fs/promises') = await import('fs/promises')
            bytes = fs.readFile(input)
        } else {
            bytes = fetch(input)
                .then(response =>
                    response.arrayBuffer()
                        .then(buffer => {
                            if (response.ok) {
                                return new Uint8Array(buffer)
                            } else {
                                const body = new TextDecoder('utf-8').decode(buffer)
                                throw new Error(`loadLanguage failed with status ${response.status}.\n\n${body}`)
                            }
                        })
                )
        }
    }

    const mod = await Module.loadWebAssemblyModule(await bytes, { loadAsync: true })
    const symbolNames = Object.keys(mod)
    const langFuncName = symbolNames.find(key => key.startsWith('syntastica_lang_'))
    if (!langFuncName) {
        console.log(`Couldn't find language function in WASM file. Symbols:\n${JSON.stringify(symbolNames, null, 2)}`)
        throw new Error('loadLanguage failed: no language function found in WASM file')
    }
    const langInfoPtr = mod[langFuncName]()
    Module._add_language(langInfoPtr)
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
 * The language must have been loaded previously by calling {@link loadLanguage}.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`. Alternatively, a {@link Theme | custom theme} can be specified.
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
 * See {@link https://rubixdev.github.io/syntastica/syntastica/renderer/struct.HtmlRenderer.html | here} for
 * more information on the output.
 */
export function highlight(
    code: string,
    language: string,
    theme: Theme | BuiltinTheme,
    renderer: string = 'HTML',
): string {
    checkModule()
    const errmsgPtr = Module._malloc(PTR_SIZE)
    const codePtr = Module.stringToNewUTF8(code)
    const languagePtr = Module.stringToNewUTF8(language)
    const themePtr = Module.stringToNewUTF8(
        JSON.stringify(theme instanceof Theme ? { custom: theme.styles } : { builtin: theme }),
    )
    const rendererPtr = Module.stringToNewUTF8(renderer)

    const resultPtr = Module._highlight(errmsgPtr, codePtr, languagePtr, themePtr, rendererPtr)
    if (resultPtr === 0) {
        const errPtr = Module.getValue(errmsgPtr, 'i8*')
        const err = Module.UTF8ToString(errPtr)
        Module._free(errPtr)
        Module._free(errmsgPtr)
        Module._free(codePtr)
        Module._free(languagePtr)
        Module._free(themePtr)
        Module._free(rendererPtr)
        Module._free(resultPtr)
        throw new Error(err)
    }
    const result = Module.UTF8ToString(resultPtr)

    Module._free(errmsgPtr)
    Module._free(codePtr)
    Module._free(languagePtr)
    Module._free(themePtr)
    Module._free(rendererPtr)
    Module._free(resultPtr)

    return result
}

/**
 * Prepare code for rendering multiple times.
 *
 * @param code - The code to highlight.
 *
 * @param language - The name of the code's language.
 *
 * The language must have been loaded previously by calling {@link loadLanguage}.
 *
 * @returns Highlight information about the code to be used by {@link render}.
 */
export function process(code: string, language: string): Highlights {
    checkModule()
    const errmsgPtr = Module._malloc(PTR_SIZE)
    const codePtr = Module.stringToNewUTF8(code)
    const languagePtr = Module.stringToNewUTF8(language)

    const resultPtr = Module._process(errmsgPtr, codePtr, languagePtr)
    if (resultPtr === 0) {
        const errPtr = Module.getValue(errmsgPtr, 'i8*')
        const err = Module.UTF8ToString(errPtr)
        Module._free(errPtr)
        Module._free(errmsgPtr)
        Module._free(codePtr)
        Module._free(languagePtr)
        Module._free(resultPtr)
        throw new Error(err)
    }
    const result: Highlights = JSON.parse(Module.UTF8ToString(resultPtr))

    Module._free(errmsgPtr)
    Module._free(codePtr)
    Module._free(languagePtr)

    return result
}

/**
 * Render code that was previously processed by calling {@link process}.
 *
 * @param highlights - The processed highlight information to render.
 *
 * @param theme - The name of the theme to use.
 *
 * All themes from {@link https://rubixdev.github.io/syntastica/syntastica_themes/ | the default collection}
 * are supported. The theme name is equivalent to its Rust path specifier, so for example the gruvbox dark theme
 * is named `gruvbox::dark`. Alternatively, a {@link Theme | custom theme} can be specified.
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
export function render(highlights: Highlights, theme: Theme | BuiltinTheme, renderer: string = 'HTML'): string {
    checkModule()
    const errmsgPtr = Module._malloc(PTR_SIZE)
    const themePtr = Module.stringToNewUTF8(
        JSON.stringify(theme instanceof Theme ? { custom: theme.styles } : { builtin: theme }),
    )
    const rendererPtr = Module.stringToNewUTF8(renderer)
    const highlightsPtr = Module.stringToNewUTF8(JSON.stringify(highlights))

    const resultPtr = Module._render(errmsgPtr, highlightsPtr, themePtr, rendererPtr)
    if (resultPtr === 0) {
        const errPtr = Module.getValue(errmsgPtr, 'i8*')
        const err = Module.UTF8ToString(errPtr)
        Module._free(errPtr)
        Module._free(errmsgPtr)
        Module._free(themePtr)
        Module._free(rendererPtr)
        Module._free(highlightsPtr)
        Module._free(resultPtr)
        throw new Error(err)
    }
    const result = Module.UTF8ToString(resultPtr)

    Module._free(errmsgPtr)
    Module._free(themePtr)
    Module._free(rendererPtr)
    Module._free(highlightsPtr)
    Module._free(resultPtr)

    return result
}

export default { init, loadLanguage, highlight, process, render, Theme }

// DISCLAIMER: All code below this line is generated with `cargo xtask codegen js-list`
// in the syntastica workspace. Do not edit this code manually!
/**
 * A list of all builtin themes.
 *
 * @see The {@link BuiltinTheme} type.
 */
export const BUILTIN_THEMES = [
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
    'github::light',
    'github::light_colorblind',
    'github::light_default',
    'github::light_high_contrast',
    'github::light_tritanopia',
    'gruvbox::dark',
    'gruvbox::light',
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
