;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-luap/c134aaec6acf4fa95fe4aa0dc9aba3eacdbbe55a/queries/highlights.scm
; adapted from https://github.com/vhyrro/tree-sitter-luap
(".") @character

[
  (anchor_begin)
  (anchor_end)
] @string.escape

[
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

[
  (zero_or_more)
  (shortest_zero_or_more)
  (one_or_more)
  (zero_or_one)
] @operator

(range
  from: (character) @constant
  "-" @punctuation.delimiter
  to: (character) @constant
)

(set
  (character) @constant
)

(class) @keyword

(negated_set
  "^" @string.regex
  (character) @constant
)

(balanced_match
  (character) @parameter
)
