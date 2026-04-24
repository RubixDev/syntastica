;; Forked from https://raw.githubusercontent.com/camdencheek/tree-sitter-go-mod/2e886870578eeba1927a2dc4bd2e2b3f598c5f9a/queries/highlights.scm
[
  "require"
  "replace"
  "go"
  "toolchain"
  "exclude"
  "retract"
  "module"
] @keyword

"=>" @operator

(comment) @comment

[
  (version)
  (go_version)
] @string
