;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-ron/78938553b93075e638035f624973083451b29055/queries/highlights.scm
; Structs
;------------
(enum_variant) @constant

(struct_entry
  (identifier) @property
)

(struct_entry
  (enum_variant
    (identifier) @constant
  )
)

(struct_name
  (identifier)
) @type

(unit_struct) @type.builtin

; Literals
;------------
(string) @string

(boolean) @boolean

(integer) @number

(float) @float

(char) @character

; Comments
;------------
[
  (line_comment)
  (block_comment)
] @comment @spell

; Punctuation
;------------
[
  "{"
  "}"
] @punctuation.bracket

[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  ","
  ":"
] @punctuation.delimiter

["-"] @operator

; Special
;------------
(escape_sequence) @string.escape

(ERROR) @error
