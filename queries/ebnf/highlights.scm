;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ebnf/highlights.scm
;; Licensed under the Apache License 2.0
; Simple tokens ;;;;
(terminal) @string

(special_sequence) @string.special

(integer) @number

(comment) @comment

; Identifiers ;;;;
; Allow different highlighting for specific casings
(
  (identifier) @type
  (#lua-match? @type "^%u")
)

(
  (identifier) @string.special.symbol
  (#lua-match? @string.special.symbol "^%l")
)

(
  (identifier) @constant
  (#lua-match? @constant "^%u[%u%d_]+$")
)

; Punctuation ;;;;
[
  ";"
  ","
] @punctuation.delimiter

[
  "|"
  "*"
  "-"
] @operator

"=" @keyword.operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket
