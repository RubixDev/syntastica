export interface SyntasticaModule extends EmscriptenModule {
    _add_language(infoPtr: number): void
    _highlight(errmsgPtr: number, codePtr: number, langPtr: number, themePtr: number, rendererPtr: number): number
    _process(errmsgPtr: number, codePtr: number, langPtr: number): number
    _render(errmsgPtr: number, highlightsPtr: number, themePtr: number, rendererPtr: number): number
    _get_builtin_theme(errmsgPtr: number, themePtr: number): number

    getValue: typeof getValue
    setValue: typeof setValue
    stringToNewUTF8: typeof allocateUTF8
    UTF8ToString: typeof UTF8ToString

    loadWebAssemblyModule(
        binary: Uint8Array,
        flags: {
            allowUndefined?: boolean
            loadAsync?: boolean
            global?: boolean
            nodelete?: boolean
        },
        libName?: string,
        localScope?: Record<string, any>,
        handle?: number,
    ): Promise<Record<string, () => number>>
}

declare const module: EmscriptenModuleFactory<SyntasticaModule>
export = module
