;; Forked from https://github.com/rush-rs/tree-sitter-wasm-queries/blob/main/queries/wat/highlights.scm
;; Licensed under the MIT license
[
  (int)
  (nat)
] @number

(float) @float

(identifier) @symbol

(string) @string

(escape_sequence) @string.escape

[
  (comment_line)
  (comment_block)
] @comment

(value_type) @type.builtin

(instr_plain) @function.call

[
  "module"
  "func"
  "type"
  "export"
  "import"
  "param"
  "result"
  "mut"
  "local"
  "if"
  "else"
  "end"
  "block"
  "loop"
  "memory"
  "global"
  "start"
] @keyword

[
  "("
  ")"
] @punctuation.bracket
