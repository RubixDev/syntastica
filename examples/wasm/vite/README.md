# `syntastica` Vite + Svelte + TypeScript Example

This folder contains an example project showcasing use of `syntastica` in
websites using Vite, Svelte, and TypeScript.

The demo can be found online
[here](https://rubixdev.github.io/syntastica/demos/vite/).

You can run a development server locally by executing these commands:

```bash
npm i
npm run dev
```

## Notes

By default, `syntastica` fails to load its WebAssembly file correctly when
running Vite's development server. To work around that, we can manually specify
the URL to the Wasm file when initializing `syntastica` and make use of Vite's
`?url` imports to get the correct URL. See [`App.svelte`](./src/App.svelte) for
more info.
