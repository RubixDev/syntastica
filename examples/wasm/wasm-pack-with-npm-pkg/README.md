# `sytnastica` wasm-pack + NPM package Example

This folder contains an example project showcasing use of `syntastica` in
websites using the `syntastica` NPM package from Rust.

The demo can be found online
[here](https://rubixdev.github.io/syntastica/demos/wasm-pack-npm/).

Using this approach you lose some static typing and a lot of flexibility, but
because `syntastica-js` is compiled with Emscripten you can make use of (almost)
all parsers, including those that use C++.

> **Note:** Currently the `syntastica` NPM package is outdated, as it would
> contain all parsers in one large binary (60+ MB). Until I figure out a way to
> make parsers opt-in and split across multiple binaries, I will not update the
> NPM package.

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
