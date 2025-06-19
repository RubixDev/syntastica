<script lang="ts">
    import syntastica, { BUILTIN_THEMES, Theme, type BuiltinTheme } from '@syntastica/core'
    import wasmUrl from '@syntastica/core/wasm?url'
    const langUrls = import.meta.glob('/node_modules/@syntastica/lang-*/*.wasm', {
        query: 'url',
        import: 'default',
    })
    const LANGUAGES = Object.keys(langUrls).map(path => path.split('/').at(-1).split('.wasm')[0])

    let loading = true
    const initPromise = syntastica.init({
        // this allows us to override the url to the Wasm file,
        // which is required for the vite dev server to load this correctly
        locateFile: () => wasmUrl,
    })
    initPromise.then(() => (loading = false))

    let code = 'fn main() {\n    println!("Hello, World!");\n}'
    let language: string = 'rust'
    let theme: BuiltinTheme = 'one::dark'

    let highlightedCode = ''
    let themeBg = 'transparent'
    const loadedLanguages = []

    $: loadLanguage(language)
    $: code, theme, updateHighlights()
    $: theme, updateThemeBg()

    async function updateHighlights() {
        await initPromise
        if (!loadedLanguages.includes(language)) return
        // in a real application, this should probably be run in a web worker to not freeze
        // the entire UI while highlighting
        highlightedCode = syntastica.highlight(code, language, theme)
    }

    async function updateThemeBg() {
        await initPromise
        const bg = Theme.fromBuiltin(theme).bg
        themeBg = bg === null ? 'transparent' : `rgb(${bg.red}, ${bg.green}, ${bg.blue})`
    }

    async function loadLanguage(lang: string) {
        if (!loadedLanguages.includes(lang)) {
            await initPromise
            loading = true
            await syntastica.loadLanguage(
                (await langUrls[`/node_modules/@syntastica/lang-${lang}/${lang}.wasm`]()) as string,
            )
            loading = false
            loadedLanguages.push(lang)
        }
        updateHighlights()
    }

    // event callback to slightly improve the default textarea editing experience
    function editorKeybinds(
        event: KeyboardEvent & { currentTarget: EventTarget & HTMLTextAreaElement },
    ) {
        const start = event.currentTarget.selectionStart
        const end = event.currentTarget.selectionEnd

        function insert(str: string, caretOffset: number = str.length) {
            // prevent default event handling
            event.preventDefault()

            // insert text
            event.currentTarget.value =
                event.currentTarget.value.substring(0, start) +
                str +
                event.currentTarget.value.substring(end)

            // put caret at the correct position
            event.currentTarget.selectionStart = event.currentTarget.selectionEnd =
                start + caretOffset

            // trigger svelte update
            code = event.currentTarget.value
        }

        if (event.key === 'Tab') {
            insert('    ') // tab inserts four spaces
        } else if (event.key === '(') {
            insert('()', 1) // bracket pairs get auto-closed
        } else if (event.key === '[') {
            insert('[]', 1)
        } else if (event.key === '{') {
            insert('{}', 1)
        } else if (
            // closing a bracket pair only moves the caret if it's already closed
            [')', ']', '}'].includes(event.key) &&
            start === end &&
            event.currentTarget.value[start] === event.key
        ) {
            event.preventDefault()
            event.currentTarget.selectionStart = event.currentTarget.selectionEnd = start + 1
        }
    }
</script>

<main>
    <div id="lang-select-container">
        <label for="language">Language:</label>
        <select name="language" id="lang-select" bind:value={language}>
            {#each LANGUAGES as lang}
                <option value={lang}>{lang}</option>
            {/each}
        </select>
    </div>
    <div id="theme-select-container">
        <label for="theme">Theme:</label>
        <select name="theme" id="theme-select" bind:value={theme}>
            {#each BUILTIN_THEMES as theme}
                <option value={theme}>{theme}</option>
            {/each}
        </select>
    </div>

    <textarea id="editor" bind:value={code} on:keydown={editorKeybinds} />
    <div id="preview" style:background-color={themeBg}>
        {#if !loading}
            {@html highlightedCode}
        {:else}
            loading...
        {/if}
    </div>
</main>

<style>
    main {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr;
        min-height: min(40rem, calc(100vh - 2rem));
        gap: 1rem;
    }

    @media screen and (max-width: 40rem) {
        main {
            grid-template-columns: 1fr;
            grid-template-rows: auto auto 1fr;
        }
    }

    #editor {
        resize: none;
        white-space: nowrap;
    }

    #preview {
        font-family: monospace;
        text-align: left;
        overflow-x: auto;
        white-space: nowrap;
        border-radius: 4px;
        padding: 4px;
    }
</style>
