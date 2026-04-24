;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-toml/64b56832c2cffe41758f28e05c756a3a98d16f41/queries/highlights.scm
; Properties
;-----------
(bare_key) @type

(quoted_key) @string

(pair
  (bare_key)
) @property

(pair
  (dotted_key
    (bare_key) @property
  )
)

; Literals
;---------
(boolean) @boolean

(comment) @comment

(string) @string

[
  (integer)
  (float)
] @number

[
  (offset_date_time)
  (local_date_time)
  (local_date)
  (local_time)
] @string.special

; Punctuation
;------------
[
  "."
  ","
] @punctuation.delimiter

"=" @operator

[
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
] @punctuation.bracket
