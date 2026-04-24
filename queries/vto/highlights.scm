;; Forked from https://raw.githubusercontent.com/ventojs/tree-sitter-vento/3b32474bc29584ea214e4e84b47102408263fe0e/queries/highlights.scm
(comment) @comment @spell

(keyword) @keyword

(tag
  [
    "{{"
    "{{-"
    "}}"
    "-}}"
  ] @punctuation.special
)

"|>" @operator
