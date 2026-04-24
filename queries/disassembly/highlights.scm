;; Forked from https://raw.githubusercontent.com/ColinKennedy/tree-sitter-disassembly/0229c0211dba909c5d45129ac784a3f4d49c243a/queries/highlights.scm
(byte) @constant

[
  (address)
  (hexadecimal)
  (integer)
] @number

(identifier) @variable

(bad_instruction) @text.warning

(code_location
  (identifier) @function.call
)

(comment) @comment

(instruction) @function

(memory_dump) @string

[
  "<"
  ">"
] @punctuation.special

[
  "+"
  ":"
] @punctuation.delimiter
