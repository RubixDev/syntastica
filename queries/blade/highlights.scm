;; Forked from https://raw.githubusercontent.com/EmranMR/tree-sitter-blade/cc764dadcbbceb3f259396fef66f970c72e94f96/queries/highlights.scm
; inherits: html
[
  (directive)
  (directive_start)
  (directive_end)
] @tag

[
  (php_tag)
  (php_end_tag)
  "{{"
  "}}"
  "{!!"
  "!!}"
  "("
  ")"
] @punctuation.bracket
