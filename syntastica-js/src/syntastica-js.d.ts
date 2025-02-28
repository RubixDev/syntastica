export interface SyntasticaModule extends EmscriptenModule {
    _init(ptr: number, len: number): void
    _highlight(code_ptr: number, lang_ptr: number, theme_ptr: number, renderer_ptr: number): number
    _process(code_ptr: number, lang_ptr: number): void
    _render(theme_ptr: number, renderer_ptr: number): number

    getValue: typeof getValue
    setValue: typeof setValue
    stringToNewUTF8: typeof allocateUTF8
    UTF8ToString: typeof UTF8ToString
}

declare const module: EmscriptenModuleFactory<SyntasticaModule>
export = module
