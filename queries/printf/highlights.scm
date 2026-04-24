;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-printf/ec4e5674573d5554fccb87a887c97d4aec489da7/queries/highlights.scm
[
  "%"
  "%%"
  (type)
] @embedded

(flags) @boolean

[
  (width)
  (precision)
] @number

(size) @type
