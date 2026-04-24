;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-chatito/c0ed82c665b732395073f635c74c300f09530a7f/queries/highlights.scm
; Punctuation
[
  "%["
  "@["
  "~["
  "*["
  "]"
  "("
  ")"
] @punctuation.bracket

[
  eq:
  _
  ","
] @punctuation.delimiter

[
  "%"
  "?"
  "#"
] @punctuation.special

; Entities
(intent) @module

(slot) @type

(variation) @variable.member

(alias) @embedded

(number) @number

(argument
  key: (string) @attribute
  value: (string) @string
)

(escape) @string.escape

; Import
"import" @keyword

(file) @string.special

; Text
(word) @markup

; Comment
(comment) @comment
