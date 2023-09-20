import syntastica from 'syntastica'

// initialize the `javascript` and `rust` languages
await syntastica.init(['javascript', 'rust'])

// highlight JavaScript input with the `one::dark` theme for the terminal with no background
console.log(syntastica.highlight('console.log("hi")', 'javascript', 'one::dark', 'terminal'))

// process some Rust input for the following `render` calls
syntastica.process('fn main() {\n    println!("Hello, World!");\n}', 'rust')

// render the Rust code once using the `one::deep` theme and no background
console.log(syntastica.render('one::deep', 'terminal'))
// and once with the `gruvbox::dark` theme and a dark gray background
console.log(syntastica.render('gruvbox::dark', 'terminal#282828'))
