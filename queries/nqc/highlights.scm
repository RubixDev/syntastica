;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-nqc/14e6da1627aaef21d2b2aa0c37d04269766dcc1d/queries/highlights.scm
; inherits: c
[
  "task"
  "sub"
] @keyword.function

["until"] @repeat

[
  "acquire"
  "monitor"
  "catch"
  "start"
  "stop"
] @keyword.coroutine
