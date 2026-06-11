# `syntastica` Dioxus Example

This folder contains an example project showcasing use of `syntastica` with
[`Dioxus`](https://dioxuslabs.com/).

The demo can be found online
[here](https://rubixdev.github.io/syntastica/demos/dioxus/).

When targeting `wasm32-unknown-unknown` as required by wasm-pack/wasm-bindgen
(which is used by dioxus), the `syntastica-parsers-git` collection must be used.

You can run a development server locally using the Dioxus CLI by executing this
command:

```bash
dx serve
```

## Notes

The main thing to look out for when targeting `wasm32-unknown-unknown` is using
the `c2rust` tree-sitter runtime. To do this you must set
`default-features = false` and enable the `runtime-c2rust` feature for all
`syntastica` dependencies. It is also recommended to only enable the parsers
that you actually need in order to keep the binary size small.
