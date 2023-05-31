<script lang="ts">
    import syntastica, { LANGUAGES, THEMES, type Language, type Theme } from 'syntastica'

    let initDone = false
    syntastica.init().then(() => (initDone = true))

    let code = 'fn main() {\n    println!("Hello, World!");\n}'
    let language: Language = 'rust'
    let theme: Theme = 'one::dark'

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
            {#each THEMES as theme}
                <option value={theme}>{theme}</option>
            {/each}
        </select>
    </div>

    <textarea id="editor" bind:value={code} on:keydown={editorKeybinds} />
    <div id="preview">
        {#if initDone}
            {@html syntastica.highlight(code, language, theme)}
        {:else}
            Initializing syntastica...
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
    }
</style>
