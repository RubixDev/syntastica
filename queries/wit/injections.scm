;; Forked from https://github.com/bytecodealliance/tree-sitter-wit/blob/main/queries/injections.scm
;; Licensed under the Apache License 2.0
(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)
