import initModule from '../pkg/syntastica-js.js'

const PTR_SIZE = Float32Array.BYTES_PER_ELEMENT

interface Module {
    _init(ptr: number, len: number): void
    _malloc(bytes: number): number
    _free(ptr: number): void
    _highlight(code_ptr: number, lang_ptr: number, theme_ptr: number): number

    UTF8ToString(ptr: number): string
    stringToNewUTF8(str: string): number
    getValue(ptr: number, type: string): number
    setValue(ptr: number, value: any, type: string): void
}

let Module: Module = null as unknown as Module

/**
 * Load the requested languages.
 *
 * This function _must_ be called before any of the others. It accepts a list of languages to load. The function can
 * be called multiple times to re-initialize with a different set of languages, but this is generally not recommended.
 *
 * @param languages - An optional list of languages to load. If `undefined`, all languages will be loaded.
 * See [here](https://rubixdev.github.io/syntastica/syntastica_parsers_git/) for a list of supported languages.
 */
export async function init(languages?: string[]) {
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
export function highlight(code: string, language: string, theme: string): string {
    const code_ptr = Module.stringToNewUTF8(code)
    const language_ptr = Module.stringToNewUTF8(language)
    const theme_ptr = Module.stringToNewUTF8(theme)

    const result_ptr = Module._highlight(code_ptr, language_ptr, theme_ptr)
    const result = Module.UTF8ToString(result_ptr)

    Module._free(code_ptr)
    Module._free(language_ptr)
    Module._free(result_ptr)

    return result
}
