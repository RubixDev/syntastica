[
  "("
  ")"
] @punctuation.bracket

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

(instr_plain) @function.call

(value_type) @type.builtin

[
  (comment_line)
  (comment_block)
] @comment

(escape_sequence) @string.escape

(string) @string

(identifier) @symbol

(float) @float

[
  (int)
  (nat)
] @number
