# `syntastica-js`

Modern and easy syntax highlighting using tree-sitter; use
[`syntastica`](https://crates.io/crates/syntastica) from JavaScript/TypeScript.

The full JavaScript/TypeScript API docs can be found
[here](https://rubixdev.github.io/syntastica/js/).

## Basic Usage

```ts
import * as syntastica from 'syntastica'

// load some languages
await syntastica.init(['rust', 'javascript'])

// highlight a piece of code once
const rustInput = `fn main() {\n    println!("Hello, World!");\n}`
const rustOutput = syntastica.highlight(rustInput, 'rust', 'one::dark')
document.getElementById('rust-code').innerHTML = rustOutput

// highlight a piece of code multiple times
const jsInput = `console.log('Hello, World!')`
syntastica.process(jsInput, 'javascript')

// - first to HTML again
const jsOutput1 = syntastica.render('gruvbox::dark')
document.getElementById('js-code').innerHTML = jsOutput1

// - then for terminals (e.g. for usage in nodejs CLIs)
const jsOutput2 = syntastica.render('one::deep', 'terminal')
console.log(jsOutput2)
```
