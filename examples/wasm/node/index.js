import syntastica from '@syntastica/core'

// initialize the syntastica
await syntastica.init()

// load the `javascript` and `rust` languages
await syntastica.loadLanguage('./node_modules/@syntastica/lang-rust/rust.wasm')
await syntastica.loadLanguage('./node_modules/@syntastica/lang-javascript/javascript.wasm')

// highlight JavaScript input with the `one::dark` theme for the terminal with no background
console.log(syntastica.highlight('console.log("hi")', 'javascript', 'one::dark', 'terminal'))

// process some Rust input for the following `render` calls
const highlights = syntastica.process('fn main() {\n    println!("Hello, World!");\n}', 'rust')

// render the Rust code once using the `one::deep` theme and no background
console.log(syntastica.render(highlights, 'one::deep', 'terminal'))
// and once with the `gruvbox::dark` theme and a dark gray background
console.log(syntastica.render(highlights, 'gruvbox::dark', 'terminal#282828'))
