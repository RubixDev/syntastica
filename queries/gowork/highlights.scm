;; Forked from https://raw.githubusercontent.com/omertuc/tree-sitter-go-work/949a8a470559543857a62102c84700d291fc984c/queries/highlights.scm
[
  "replace"
  "go"
  "use"
] @keyword

"=>" @operator

(comment) @comment

[
  (version)
  (go_version)
] @string
