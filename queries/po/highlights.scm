;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-po/bd860a0f57f697162bf28e576674be9c1500db5e/queries/highlights.scm
; Keywords
[
  "msgctxt"
  "msgid"
  "msgid_plural"
  "msgstr"
  "msgstr_plural"
] @keyword

; Punctuation
[
  "["
  "]"
] @punctuation.bracket

; Literals
(string) @string

(escape_sequence) @string.escape

(number) @number

; Comments
(comment) @comment @spell

(comment
  (reference
    (text) @string.special.path
  )
)

(comment
  (flag
    (text) @label
  )
)

; Errors
(ERROR) @error
