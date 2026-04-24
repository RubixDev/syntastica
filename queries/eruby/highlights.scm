;; Forked from https://raw.githubusercontent.com/tree-sitter/tree-sitter-embedded-template/3499d85f0a0d937c507a4a65368f2f63772786e1/queries/highlights.scm
(comment_directive) @comment

[
  "<%#"
  "<%"
  "<%="
  "<%_"
  "<%-"
  "%>"
  "-%>"
  "_%>"
] @keyword
