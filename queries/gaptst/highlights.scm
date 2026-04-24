;; Forked from https://raw.githubusercontent.com/gap-system/tree-sitter-gaptst/69086d7627c03e1f4baf766bcef14c60d9e92331/queries/highlights.scm
(output_line) @markup.raw.block

[
  "#@local"
  "#@exec"
] @keyword

[
  "gap> "
  "> "
] @keyword.debug

[
  "#@if"
  "#@else"
  "#@fi"
] @keyword.conditional
