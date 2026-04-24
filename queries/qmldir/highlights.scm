;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-qmldir/6b2b5e41734bd6f07ea4c36ac20fb6f14061c841/queries/highlights.scm
; Preproc
(command
  (identifier) @preproc
)

; Keywords
(keyword) @keyword

; Literals
(number) @number

(float) @float

; Variables
[
  (identifier)
  (unit)
] @variable

; Comments
(comment) @comment @spell
