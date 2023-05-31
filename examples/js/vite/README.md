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

By default, Vite complains about a supposedly invalid `main`/`module`/`exports`
field in `syntastica`'s `package.json`, but only when running a development
server. I could not find any error there, but if you can, please let me know.
This issue can be circumvented by manually setting the entry point in the
[`vite.config.ts`](./vite.config.ts) file.
